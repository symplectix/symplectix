//! A tiny library to parse a file like Procfile.

use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
};
use std::{
    iter,
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
pub fn parse<R>(source: R, envs: Option<HashMap<String, String>>) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    let rcfile = {
        let mut lines = Vec::new();
        let buf = io::BufReader::new(source);
        for line in buf.lines() {
            let mut line = line?;
            // Skip comments,
            if let Some(n) = line.find('#') {
                let _comment = line.split_off(n);
            }
            // and empty lines.
            if line.trim_ascii().is_empty() {
                continue;
            }
            lines.push(line);
        }
        lines
    };

    let tokens = Tokens::new(
        rcfile
            .iter()
            // nb. Each string produced by buf.lines() will not have a
            // newline byte (the `0xA` byte) or `CRLF` at the end.
            .flat_map(|line| line.trim_ascii().chars().chain(iter::once('\n'))),
        envs,
    );
    Ok(tokens.map(|flag| Entry { flag }).collect())
}

/// Transforms an input chars into a sequence of tokens.
pub struct Tokens<I> {
    lex:  Lexer<I>,
    envs: HashMap<String, String>,
}

impl<I> Tokens<I> {
    /// Creates a new Tokens.
    ///
    /// ```
    /// # use flagrc::Tokens;
    /// let mut tokens = Tokens::new("foo\"ba\"r baz".chars(), None);
    /// assert_eq!(tokens.next().unwrap(), ["foobar", "baz"]);
    /// assert_eq!(tokens.next(), None);
    /// ```
    pub fn new<T>(chars: T, envs: Option<HashMap<String, String>>) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens { lex: Lexer::new(chars), envs: envs.unwrap_or_default() }
    }
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = char>,
{
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token_to_str = |token: Token| -> String {
            let mut out = String::new();
            for word in token.words {
                match word {
                    Empty => {}
                    Lit(lit) => out.push_str(lit.as_str()),
                    Var(var) => {
                        if let Some(val) = self.envs.get(var.as_str()) {
                            out.push_str(val);
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
            .take_while_inclusive(|t| !matches!(t.words.last(), Some(Word::NewLine(_))))
            .map(token_to_str);

        let tokens = tokens.collect::<Vec<_>>();
        if tokens.is_empty() { None } else { Some(tokens) }
    }
}

struct Lexer<I> {
    chars: I,
    token: Token,
    quote: Option<char>,
}

// Token is a single element that make up a line.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Lit(String),
    /// A variable $LIKE_THIS.
    ///
    /// Not support:
    /// - bracing: ${LIKE_THIS}
    /// - nesting: $$LIKE_THIS
    Var(String),
    /// A word for the *non-escaped* newline delimiter.
    NewLine(char),
}

impl Token {
    fn new() -> Self {
        Token { words: Vec::new() }
    }

    fn reset(&mut self) -> Self {
        mem::replace(self, Token::new())
    }

    fn push(&mut self, c: char) {
        if let Some(last) = self.words.last_mut() {
            match last {
                Empty => self.words.push(Word::Lit(c.to_string())),
                Lit(s) => s.push(c),
                Var(s) => s.push(c),
                NewLine(_) => unreachable!("pushing a char onto NewLine"),
            }
        } else {
            self.words.push(Lit(c.to_string()));
        }
    }

    fn break_line(&mut self, c: char) {
        self.words.push(Word::NewLine(c));
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
        self.words.push(Var(String::new()))
    }

    fn in_var(&self) -> bool {
        matches!(self.words.last(), Some(Word::Var(_)))
    }
}

impl<I> Lexer<I> {
    fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Lexer { chars: chars.into_iter(), token: Token::new(), quote: None }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.chars.find(|c| !c.is_ascii_whitespace()).map(|c| self.next_token(c))
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn next_token(&mut self, mut c: char) -> Token {
        loop {
            if self.in_single_quote() {
                match c {
                    '\'' => {
                        self.quote = None;
                    }
                    c => {
                        self.token.push(c);
                    }
                }
            } else if self.in_double_quote() {
                match c {
                    '"' => {
                        if self.in_var() {
                            self.token.split();
                        }
                        self.quote = None;
                    }
                    '\\' if self.in_var() => {
                        self.token.split();
                        self.token.push(c);
                    }
                    '$' => {
                        self.token.begin_var();
                    }
                    c => {
                        self.token.push(c);
                    }
                }
            } else if self.in_var() {
                match c {
                    '\'' | '"' => {
                        self.quote = Some(c);
                        self.token.split();
                    }
                    '\\' => {
                        self.token.split();
                    }
                    '$' => {
                        self.token.begin_var();
                    }
                    c if c.is_ascii_whitespace() => {
                        break;
                    }
                    // Var token should satisfy either of:
                    // * is_ascii_alphanumeric(c)
                    // * '_'
                    c if c.is_ascii_alphanumeric() || c == '_' => {
                        self.token.push(c);
                    }
                    c => {
                        // c is not a part of Var.
                        self.token.split();
                        self.token.push(c);
                    }
                }
            } else {
                match c {
                    '\'' | '"' => {
                        self.quote = Some(c);
                    }
                    '\\' => {
                        if let Some(esc) = self.chars.next() {
                            // A '\' at the end of a line continues the line.
                            if !(esc == '\n' || esc == '\r') {
                                self.token.push(esc);
                            }
                        } else {
                            break;
                        }
                    }
                    '$' => {
                        self.token.begin_var();
                    }
                    '\n' | '\r' => {
                        self.token.break_line(c);
                        break;
                    }
                    c if c.is_ascii_whitespace() => {
                        break;
                    }
                    c => {
                        self.token.push(c);
                    }
                }
            }

            if let Some(next) = self.chars.next() {
                c = next;
            } else {
                break;
            }
        }
        self.output()
    }

    fn in_single_quote(&self) -> bool {
        self.quote.as_ref().is_some_and(|c| *c == '\'')
    }

    fn in_double_quote(&self) -> bool {
        self.quote.as_ref().is_some_and(|c| *c == '"')
    }

    fn in_var(&self) -> bool {
        self.token.in_var()
    }

    fn output(&mut self) -> Token {
        self.token.reset()
    }
}
