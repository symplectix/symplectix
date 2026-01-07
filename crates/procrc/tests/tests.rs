#![allow(missing_docs)]
use std::io;

use procrc::{
    Tokens,
    parse,
};

#[test]
fn empty_procrc() {
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
fn check_multiline() {
    #[rustfmt::skip]
        let rc = r#"
test0

test1 gunicorn -b \
:$PORT \
main:app \
hey!!!

test2 abc \
--kill-after 10s \
--env APPLE=banana \
-- foo \
a \
b \
c

test3 \
"C:\path" C:\path foo a=1 b=2 c=3
test4 --kill-after "10'm\n" \
--timeout-is-ok -- \
foo a 1 b 2 c 3

test5 a b \
c

test6 a  b \c \d \
e

test7 a  b \c \ \
\d e
"#;

    for entry in parse(io::Cursor::new(rc)).expect("reading from a cursor never fails") {
        println!("{entry:?}");
    }
}

#[test]
fn test_get_next_tokens() {
    let mut tokens = Tokens::new("".chars());
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

    let mut tokens = Tokens::new("\"foo ; bar\"".chars());
    assert_eq!(tokens.next().unwrap(), "foo ; bar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Tokens::new("foo ; bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), ";");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);
}
