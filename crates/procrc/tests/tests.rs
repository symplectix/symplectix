#![allow(missing_docs)]
use std::io;

use procrc::{
    Tokens,
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

test2 abc \
-x 10s \
--env A"PPL"E=banana \
-- foo \
a \
b \
c

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

    assert_eq!(entries[0].cmdline, ["test0"]);
    assert_eq!(entries[1].cmdline, ["test1", "gunicorn", "-b", ":$PORT", "main:app"]);
    assert_eq!(
        entries[2].cmdline,
        ["test2", "abc", "-x", "10s", "--env", "APPLE=banana", "--", "foo", "a", "b", "c"]
    );
    assert_eq!(entries[3].cmdline, ["test3", "C:\\path", "D:ath"]);
    assert_eq!(entries[4].cmdline, ["test4", "10m\\n", "--", "foo", "a=1"]);
    assert_eq!(entries[5].cmdline, ["test5", "a", "b", "c"]);
    assert_eq!(entries[6].cmdline, ["test6", "a", "b", "e"]);
    assert_eq!(entries[7].cmdline, ["test7", "a", "b", "e"]);
}

#[test]
fn test_get_next_tokens() {
    let mut tokens = Tokens::new("".chars());
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new(" ".chars());
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("\\ ".chars());
    assert_eq!(tokens.next().unwrap(), "");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("'\ne'".chars());
    assert_eq!(tokens.next().unwrap(), "\ne");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("\\\ne".chars());
    assert_eq!(tokens.next().unwrap(), "e");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("'A\"PPL\"E'".chars());
    assert_eq!(tokens.next().unwrap(), "A\"PPL\"E");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("A\"PPL\"E".chars());
    assert_eq!(tokens.next().unwrap(), "APPLE");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("foo bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("foo\"ba\"r 10s".chars());
    assert_eq!(tokens.next().unwrap(), "foobar");
    assert_eq!(tokens.next().unwrap(), "10s");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("foo\"bar".chars());
    assert_eq!(tokens.next().unwrap(), "foobar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("FOO=\"10's\"".chars());
    assert_eq!(tokens.next().unwrap(), "FOO=10's");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("\"foo\nbar\" ${BAR}".chars());
    assert_eq!(tokens.next().unwrap(), "foo\nbar");
    assert_eq!(tokens.next().unwrap(), "${BAR}");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("foo ; bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), ";");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);
}
