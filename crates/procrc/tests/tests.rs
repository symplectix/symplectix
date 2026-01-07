#![allow(missing_docs)]
use std::io;

use procrc::{
    Entry,
    Expand,
    Tokens,
    parse,
    parse_multiline,
};

#[test]
fn check_parse() {
    let procfile = r#"
# comment
  # comment
    # comment
s0:
s1: gunicorn -b :$PORT main:app
s2: abc --kill-after 10s --env "APP"LE=banana -- foo a b c
s3: --backslask C:\path foo a=1 b=2 c=3
s4: --kill-after 10m --timeout-is-ok -- foo a 1 b 2 c 3
s5: a b c
s6: a  b \c \d e
s7: a  b \ \ c \d e
"#;
    let entries = parse(io::Cursor::new(procfile)).unwrap();
    for entry in entries {
        println!("{entry:?}");
    }
}

#[test]
fn empty() {
    let procfile = r#""#;
    assert!(parse(io::Cursor::new(procfile)).is_ok());

    let procfile = r#"
"#;
    assert!(parse(io::Cursor::new(procfile)).is_ok());

    let procfile = r#"

        "#;
    assert!(parse(io::Cursor::new(procfile)).is_ok());
}

#[test]
fn check_multiline() {
    #[rustfmt::skip]
        let procfile = r#"
m0:

m1: gunicorn -b \
:$PORT\
main:app

m2: abc \
--kill-after 10s \
--env APPLE=banana \
-- foo a b c

m3: --backslask \
C:\path foo a=1 b=2 c=3

m4: --kill-after "10'm
" --timeout-is-ok -- \

foo a 1 b 2 c 3      \

m5: a b c

m6: a  b \c \d \
e

m6: a  b \c \ \
\d e
"#;

    let entries = parse_multiline(io::Cursor::new(procfile)).unwrap();
    for entry in entries {
        println!("{entry:?}");
    }
}

#[test]
fn test_get_next_tokens() {
    let mut tokens = Expand::new("".chars());
    assert_eq!(tokens.next(), None);

    let mut tokens = Expand::new("foo bar".chars());
    assert_eq!(tokens.next().unwrap(), "foo");
    assert_eq!(tokens.next().unwrap(), "bar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Expand::new("foo\"ba\"r 10s".chars());
    assert_eq!(tokens.next().unwrap(), "foobar");
    assert_eq!(tokens.next().unwrap(), "10s");
    assert_eq!(tokens.next(), None);

    let mut tokens = Expand::new("foo\"bar".chars());
    assert_eq!(tokens.next().unwrap(), "foobar");
    assert_eq!(tokens.next(), None);

    let mut tokens = Expand::new("FOO=\"10's\"".chars());
    assert_eq!(tokens.next().unwrap(), "FOO=10's");
    assert_eq!(tokens.next(), None);
}
