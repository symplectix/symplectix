#![allow(missing_docs)]
use std::io;

use procrc::{
    Parser,
    parse,
};

#[test]
fn no_entries() {
    let rc = r#""#;
    assert!(parse(io::Cursor::new(rc)).is_ok());

    let rc = r#"
"#;
    assert!(parse(io::Cursor::new(rc)).is_ok());

    let rc = r#"

        "#;
    assert!(parse(io::Cursor::new(rc)).is_ok());
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
\c e
"#;

    let entries = parse(io::Cursor::new(rc)).expect("reading from a cursor never fails");

    assert_eq!(entries[0].flag, ["test0"]);
    assert_eq!(entries[1].flag, ["test1", "gunicorn", "-b", ":$PORT", "main:app"]);
    assert_eq!(entries[2].flag, ["test2", "-x", "10s", "--env", "ABC=def", "--", "foo", "bar"]);
    assert_eq!(entries[3].flag, ["test3", "C:\\path", "D:ath"]);
    assert_eq!(entries[4].flag, ["test4", "10m\\n", "--", "foo", "a=1"]);
    assert_eq!(entries[5].flag, ["test5", "a", "b", "c"]);
    assert_eq!(entries[6].flag, ["test6", "a", "b", "e"]);
    assert_eq!(entries[7].flag, ["test7", "a", "b", "e"]);
}

fn single_token(chars: impl IntoIterator<Item = char>) -> Option<String> {
    let mut tokens = Parser::new(chars);
    let next = tokens.next();
    // Check no more tokens.
    assert_eq!(tokens.next(), None);
    next
}

#[test]
fn get_single_token() {
    assert_eq!(single_token("".chars()), None);
    assert_eq!(single_token(" ".chars()), None);

    assert_eq!(single_token("\\ ".chars()).unwrap(), "");
    assert_eq!(single_token("'\ne'".chars()).unwrap(), "\ne");
    assert_eq!(single_token("\\\ne".chars()).unwrap(), "e");

    assert_eq!(single_token("A\"PPL\"E".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("\"APPL\"E".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("A\"PPLE\"".chars()).unwrap(), "APPLE");

    assert_eq!(single_token("'A\"PPL\"E'".chars()).unwrap(), "A\"PPL\"E");
    assert_eq!(single_token("'\"APPL\"E'".chars()).unwrap(), "\"APPL\"E");
    assert_eq!(single_token("'A\"PPLE\"'".chars()).unwrap(), "A\"PPLE\"");
}

#[test]
fn ignore_whitespaces() {
    assert_eq!(single_token("     A\"PPL\"E".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("     \"APPL\"E".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("     A\"PPLE\"".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("A\"PPL\"E     ".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("\"APPL\"E     ".chars()).unwrap(), "APPLE");
    assert_eq!(single_token("A\"PPLE\"     ".chars()).unwrap(), "APPLE");
}

#[test]
fn delimited_in_quote() {
    assert_eq!(single_token("\"foobar  baz\"".chars()).unwrap(), "foobar  baz");
}

#[test]
fn quote_in_another_quote() {
    assert_eq!(single_token("foo=\"1'0'1\"".chars()).unwrap(), "foo=1'0'1");
}

#[test]
fn no_matching_quote() {
    assert_eq!(single_token("foo\"bar".chars()).unwrap(), "foobar");
}

#[test]
fn get_tokens() {
    let mut tokens = Parser::new("foo bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Parser::new("foo\"ba\"r 10s".chars());
    assert_eq!(tokens.next().unwrap(), "foobar");
    assert_eq!(tokens.next().unwrap(), "10s");
    assert_eq!(tokens.next(), None);

    let mut tokens = Parser::new("\"foo\nbar\" ${BAR}".chars());
    assert_eq!(tokens.next().unwrap(), "foo\nbar");
    assert_eq!(tokens.next().unwrap(), "${BAR}");
    assert_eq!(tokens.next(), None);

    let mut tokens = Parser::new("foo ; bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), ";");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);
}
