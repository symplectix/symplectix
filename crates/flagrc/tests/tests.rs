#![allow(missing_docs)]
use std::collections::HashMap;
use std::io;

use flagrc::{
    Entry,
    Tokens,
    parse,
};

#[test]
fn no_entries() {
    let rc = r#""#;
    assert!(parse(io::Cursor::new(rc), None).is_ok());

    let rc = r#"
"#;
    assert!(parse(io::Cursor::new(rc), None).is_ok());

    let rc = r#"

        "#;
    assert!(parse(io::Cursor::new(rc), None).is_ok());
}

#[test]
fn many_entries() {
    #[rustfmt::skip]
        let rc = r#"
test0 # comment

test1 gunicorn -b \ # comment
:$PORT \
main:app

test2 \
-x 10s \
--env A"B"C=def \
-- foo \ bar

test3 \
"C:\path" D:\path

test4 "10m\n" \
           -- \
foo a=1

test5 a b \ \ c

test6 a  b \c \d \
e

test7 a  b \c \ \
\d e
"#;

    let envs = {
        let mut envs = HashMap::new();
        envs.insert("PORT".to_owned(), "8080".to_owned());
        Some(envs)
    };
    let entries = parse(io::Cursor::new(rc), envs).expect("reading from a cursor never fails");

    assert_eq!(entries[0].flag, ["test0"]);
    assert_eq!(entries[1].flag, ["test1", "gunicorn", "-b", ":8080", "main:app"]);
    assert_eq!(entries[2].flag, ["test2", "-x", "10s", "--env", "ABC=def", "--", "foo", " bar"]);
    assert_eq!(entries[3].flag, ["test3", "C:\\path", "D:path"]);
    assert_eq!(entries[4].flag, ["test4", "10m\\n", "--", "foo", "a=1"]);
    assert_eq!(entries[5].flag, ["test5", "a", "b", "  c"]);
    assert_eq!(entries[6].flag, ["test6", "a", "b", "c", "d", "e"]);
    assert_eq!(entries[7].flag, ["test7", "a", "b", "c", " d", "e"]);
}

fn token(chars: impl IntoIterator<Item = char>) -> Vec<String> {
    let envs = {
        let mut envs = HashMap::new();
        envs.insert("PORT".to_owned(), "8080".to_owned());
        envs.insert("TEST".to_owned(), "ch".to_owned());
        Some(envs)
    };

    let mut tokens = Tokens::new(chars, envs);
    let f = tokens.next().expect("no tokens");
    // Check no more tokens.
    assert_eq!(tokens.next(), None);
    f.flag
}

#[test]
fn empty_token() {
    assert_eq!(token("".chars()), [""; 0]);
    assert_eq!(token(" ".chars()), [""; 0]);
}

#[test]
fn get_single_token() {
    assert_eq!(token("\\ ".chars()), [" "]);
    assert_eq!(token("\\ e".chars()), [" e"]);
    assert_eq!(token("'\ne'".chars()), ["\ne"]);

    assert_eq!(token("A\"PPL\"E".chars()), ["APPLE"]);
    assert_eq!(token("\"APPL\"E".chars()), ["APPLE"]);
    assert_eq!(token("A\"PPLE\"".chars()), ["APPLE"]);

    assert_eq!(token("'A\"PPL\"E'".chars()), ["A\"PPL\"E"]);
    assert_eq!(token("'\"APPL\"E'".chars()), ["\"APPL\"E"]);
    assert_eq!(token("'A\"PPLE\"'".chars()), ["A\"PPLE\""]);

    assert_eq!(token("A\\e".chars()), ["Ae"]);
    assert_eq!(token("\"'A\\e\"".chars()), ["'A\\e"]);
}

#[test]
fn get_tokens() {
    assert_eq!(token("foo bar".chars()), ["foo", "bar"]);
    assert_eq!(token("foo\"ba\"r baz".chars()), ["foobar", "baz"]);
    assert_eq!(token("\"foo\nbar\" baz".chars()), ["foo\nbar", "baz"]);
    assert_eq!(token("foo; bar; baz".chars()), ["foo;", "bar;", "baz"]);
    assert_eq!(token("'foo; bar'; baz".chars()), ["foo; bar;", "baz"]);

    assert_eq!(token("a b".chars()), ["a", "b"]);
    assert_eq!(token("a \\ b".chars()), ["a", " b"]);
    assert_eq!(token("a \\\nb".chars()), ["a", "b"]);

    // Shell prints "for" in this case.
    // https://aosabook.org/en/v1/bash.html
    //
    // But procrc does not support defining a new var.
    assert_eq!(
        token("for for in for; do for=for; done; echo $for".chars()),
        ["for", "for", "in", "for;", "do", "for=for;", "done;", "echo", ""],
    );
}

#[test]
fn ignore_whitespaces() {
    assert_eq!(token("     A\"PPL\"E".chars()), ["APPLE"]);
    assert_eq!(token("     \"APPL\"E".chars()), ["APPLE"]);
    assert_eq!(token("     A\"PPLE\"".chars()), ["APPLE"]);
    assert_eq!(token("A\"PPL\"E     ".chars()), ["APPLE"]);
    assert_eq!(token("\"APPL\"E     ".chars()), ["APPLE"]);
    assert_eq!(token("A\"PPLE\"     ".chars()), ["APPLE"]);
}

#[test]
fn delimited_in_quote() {
    assert_eq!(token("\"foobar  baz\"".chars()), ["foobar  baz"]);
}

#[test]
fn quote_in_another_quote() {
    assert_eq!(token("foo=\"1'0'1\"".chars()), ["foo=1'0'1"]);
}

#[test]
fn no_matching_quote() {
    assert_eq!(token("foo\"bar".chars()), ["foobar"]);
}

#[test]
fn expand_envs() {
    assert_eq!(token("\\$TEST".chars()), ["$TEST"]);
    assert_eq!(token("$TEST".chars()), ["ch"]);
    assert_eq!(token("$TEST#a".chars()), ["ch#a"]);
    assert_eq!(token("$TEST-a".chars()), ["ch-a"]);
    assert_eq!(token("e\"$TEST\"o".chars()), ["echo"]);
    assert_eq!(token("$TEST\\A".chars()), ["chA"]);
    assert_eq!(token("\"$TEST\\A\"".chars()), ["ch\\A"]);
    assert_eq!(token("$TEST{cc".chars()), ["ch{cc"]);
    assert_eq!(token("$TEST\"cc\"".chars()), ["chcc"]);
    assert_eq!(token("e\"$TEST\"o hello".chars()), ["echo", "hello"]);
}
