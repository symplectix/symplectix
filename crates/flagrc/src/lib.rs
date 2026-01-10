//! A tiny library to parse a file like Procfile.

use std::borrow::{
    Borrow,
    Cow,
};
use std::collections::HashMap;
use std::io::{
    self,
    Read,
};
use std::{
    fmt,
    mem,
};

use Word::*;
use itertools::Itertools;

/// Procrc entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Indicates the command that you should execute such as:
    /// - `gunicorn -b :$PORT main:app`
    /// - `rake jobs:work`
    pub flag: Vec<String>,
}

/// Parses procrc.
pub fn parse<R>(source: R, vars: Option<HashMap<String, String>>) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    let buf = io::BufReader::new(source);
    let tokens = Tokens::new(buf.bytes().map_while(Result::ok), vars);
    Ok(tokens.map(|flag| Entry { flag }).collect())
}

/// Transforms an input chars into a sequence of tokens.
pub struct Tokens<I> {
    lex:  Lexer<I>,
    vars: HashMap<String, String>,
}

impl<I> Tokens<I> {
    /// Creates a new Tokens.
    ///
    /// ```
    /// # use flagrc::Tokens;
    /// let mut tokens = Tokens::new("foo\"ba\"r baz".bytes(), None);
    /// assert_eq!(tokens.next().unwrap(), ["foobar", "baz"]);
    /// assert_eq!(tokens.next(), None);
    /// ```
    pub fn new<T>(chars: T, vars: Option<HashMap<String, String>>) -> Self
    where
        T: IntoIterator<Item = u8, IntoIter = I>,
    {
        Tokens { lex: Lexer::new(chars), vars: vars.unwrap_or_default() }
    }
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token_to_str = |token: Token| {
            let mut out = String::new();
            for word in token.words {
                match word {
                    Empty => {}
                    Lit(v) => {
                        let lit: Cow<str> = String::from_utf8_lossy(&v);
                        out.push_str(lit.borrow());
                    }
                    Var(v) => {
                        let var: Cow<str> = String::from_utf8_lossy(&v);
                        let var: &str = var.borrow();
                        if let Some(var) = self.vars.get(var) {
                            out.push_str(var);
                        }
                    }
                    NewLine(_) => {
                        break;
                    }
                }
            }
            out
        };
        let tokens = self
            .lex
            .by_ref()
            // TODO: Use iter::take_until or something.
            // Rust stdlib may have this someday.
            // https://github.com/rust-lang/rust/issues/62208
            .take_while_inclusive(|t| !matches!(t.words.last(), Some(NewLine(_))))
            .map(token_to_str);

        let tokens = tokens.collect::<Vec<_>>();
        if tokens.is_empty() { None } else { Some(tokens) }
    }
}

struct Lexer<I> {
    bytes: I,
    token: Token,
    quote: Option<u8>,
}

// Token is a single element that make up a line.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    words: Vec<Word>,
}

#[derive(Clone, PartialEq, Eq)]
enum Word {
    /// Empty is a dummy word that disappears when rendering a token as a string. It can be used
    /// for the following purposes:
    /// 1. As an empty word (of course).
    /// 2. To split words within a single token.
    Empty,
    /// A literal string.
    ///
    /// For examples:
    /// - Lit("$x") is just a string "$x", not a variable.
    /// - Lit("\n") is just a string "\n", not a line break.
    Lit(Vec<u8>),
    /// A variable $LIKE_THIS.
    ///
    /// Not support:
    /// - bracing: ${LIKE_THIS}
    /// - nesting: $$LIKE_THIS
    Var(Vec<u8>),
    /// A word for the *non-escaped* newline delimiter.
    NewLine(u8),
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Empty => f.debug_tuple("Empty").finish(),
            Lit(v) => f.debug_tuple("Lit").field(&String::from_utf8_lossy(v)).finish(),
            Var(v) => f.debug_tuple("Var").field(&String::from_utf8_lossy(v)).finish(),
            NewLine(b) => f.debug_tuple("NewLine").field(&(*b as char)).finish(),
        }
    }
}

impl Token {
    fn new() -> Self {
        Token { words: Vec::new() }
    }

    fn reset(&mut self) -> Self {
        mem::replace(self, Token::new())
    }

    fn push(&mut self, b: u8) {
        if let Some(last) = self.words.last_mut() {
            match last {
                Empty => self.words.push(Lit(vec![b])),
                Lit(lit) => lit.push(b),
                Var(var) => var.push(b),
                NewLine(_) => unreachable!("pushing a byte onto NewLine"),
            }
        } else {
            self.words.push(Lit(vec![b]));
        }
    }

    fn break_line(&mut self, b: u8) {
        self.words.push(Word::NewLine(b));
    }

    fn split(&mut self) {
        if let Some(last) = self.words.last() {
            if !(matches!(last, Empty)) {
                self.words.push(Empty);
            }
        } else {
            self.words.push(Empty);
        }
    }

    fn begin_var(&mut self) {
        self.words.push(Var(Vec::new()))
    }

    fn in_var(&self) -> bool {
        matches!(self.words.last(), Some(Word::Var(_)))
    }
}

impl<I> Lexer<I> {
    fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = u8, IntoIter = I>,
    {
        Lexer { bytes: chars.into_iter(), token: Token::new(), quote: None }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.bytes.find(|b| !b.is_ascii_whitespace()).map(|b| self.next_token(b))
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = u8>,
{
    fn next_token(&mut self, mut b: u8) -> Token {
        loop {
            if self.in_single_quote() {
                match b {
                    b'\'' => {
                        self.quote = None;
                    }
                    b => {
                        self.token.push(b);
                    }
                }
            } else if self.in_double_quote() {
                match b {
                    b'"' => {
                        if self.in_var() {
                            self.token.split();
                        }
                        self.quote = None;
                    }
                    b'\\' if self.in_var() => {
                        self.token.split();
                        self.token.push(b);
                    }
                    b'$' => {
                        self.token.begin_var();
                    }
                    b => {
                        self.token.push(b);
                    }
                }
            } else if self.in_var() {
                match b {
                    b'\'' | b'"' => {
                        self.quote = Some(b);
                        self.token.split();
                    }
                    b'\\' => {
                        self.token.split();
                    }
                    b'$' => {
                        self.token.begin_var();
                    }
                    b if b.is_ascii_whitespace() => {
                        break;
                    }
                    // Var token should satisfy either of:
                    // * is_ascii_alphanumeric(c)
                    // * '_'
                    b if b.is_ascii_alphanumeric() || b == b'_' => {
                        self.token.push(b);
                    }
                    b => {
                        // c is not a part of Var.
                        self.token.split();
                        self.token.push(b);
                    }
                }
            } else {
                match b {
                    b'\'' | b'"' => {
                        self.quote = Some(b);
                    }
                    b'\\' => {
                        if let Some(esc) = self.bytes.next() {
                            // A '\' at the end of a line continues the line.
                            if !(esc == b'\r' || esc == b'\n') {
                                self.token.push(esc);
                            }
                        } else {
                            break;
                        }
                    }
                    b'$' => {
                        self.token.begin_var();
                    }
                    b if b.is_ascii_whitespace() => {
                        if b == b'\n' {
                            self.token.break_line(b);
                        }
                        break;
                    }
                    b => {
                        self.token.push(b);
                    }
                }
            }

            if let Some(next) = self.bytes.next() {
                b = next;
            } else {
                break;
            }
        }
        self.output()
    }

    fn in_single_quote(&self) -> bool {
        self.quote.as_ref().is_some_and(|b| *b == b'\'')
    }

    fn in_double_quote(&self) -> bool {
        self.quote.as_ref().is_some_and(|b| *b == b'"')
    }

    fn in_var(&self) -> bool {
        self.token.in_var()
    }

    fn output(&mut self) -> Token {
        self.token.reset()
    }
}
