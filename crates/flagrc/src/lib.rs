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
    let rc = {
        let mut vec = Vec::new();
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
            vec.push(line);
        }
        vec
    };

    let mut rc_lines = rc.iter();
    let get_entry = || -> Option<Entry> {
        let tokens = Tokens::new(
            // The all lines in the chunk are collected together to form
            // a single Entry.
            rc_lines
                .by_ref()
                // A '\' at the end of a line continues the line.
                //
                // TODO: Use iter::take_until or something.
                // Rust stdlib may have this someday.
                // https://github.com/rust-lang/rust/issues/62208
                .take_while_inclusive(|line| line.trim_ascii().ends_with('\\'))
                // nb. Each string produced by buf.lines() will not have a
                // newline byte (the `0xA` byte) or `CRLF` at the end.
                .flat_map(|line| line.trim_ascii().chars().chain(iter::once('\n'))),
            envs.as_ref(),
        )
        // Tokens emits `Some("")` when input is eg. "\\ ".
        // This behavior is important in the current implementation:
        // if input is something like "\\ x", returning None will not output x.
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>();

        if tokens.is_empty() { None } else { Some(Entry { flag: tokens }) }
    };
    Ok(iter::from_fn(get_entry).collect())
}

/// Transforms an input chars into a sequence of tokens.
pub struct Tokens<'e, I>
where
    I: Iterator,
{
    lex:  Lexer<I>,
    envs: Option<&'e HashMap<String, String>>,
}

impl<'e, I> Tokens<'e, I>
where
    I: Iterator<Item = char>,
{
    /// Creates a new Tokens.
    ///
    /// ```
    /// # use flagrc::Tokens;
    /// let mut tokens = Tokens::new("foo\"ba\"r baz".chars(), None);
    /// assert_eq!(tokens.next().unwrap(), "foobar");
    /// assert_eq!(tokens.next().unwrap(), "baz");
    /// assert_eq!(tokens.next(), None);
    /// ```
    pub fn new<T>(chars: T, envs: Option<&'e HashMap<String, String>>) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens { lex: Lexer::new(chars), envs }
    }
}

impl<'e, I> Iterator for Tokens<'e, I>
where
    I: Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex.next().map(|token| {
            let mut out = String::new();
            for word in token.words {
                match word {
                    Word::Sep => {}
                    Word::Lit(lit) => {
                        out.push_str(lit.as_str());
                    }
                    Word::Var(var) => {
                        if let Some(val) = self.envs.as_ref().and_then(|es| es.get(var.as_str())) {
                            out.push_str(val);
                        }
                    }
                }
            }
            out
        })
    }
}

struct Lexer<I>
where
    I: Iterator,
{
    chars: iter::Peekable<I>,
    token: Token,
    quote: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Word {
    Sep,
    Lit(String),
    Var(String),
}

impl Token {
    fn new() -> Self {
        Token { words: Vec::new() }
    }

    fn reset(&mut self) -> Self {
        mem::replace(self, Token::new())
    }

    fn push(&mut self, c: char) {
        if let Some(word) = self.words.last_mut() {
            match word {
                Word::Lit(s) => s.push(c),
                Word::Var(s) => s.push(c),
                Word::Sep => self.words.push(Word::Lit(c.to_string())),
            }
        } else {
            self.words.push(Word::Lit(c.to_string()));
        }
    }

    fn new_sep(&mut self) {
        self.words.push(Word::Sep)
    }

    fn new_var(&mut self) {
        self.words.push(Word::Var(String::new()))
    }
}

impl<I> Lexer<I>
where
    I: Iterator,
{
    fn new<T>(chars: T) -> Self
    where
        I: Iterator<Item = char>,
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Lexer { chars: chars.into_iter().peekable(), token: Token::new(), quote: None }
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
                        self.token.new_sep();
                    }
                    c => {
                        self.token.push(c);
                    }
                }
            } else if self.in_double_quote() {
                match c {
                    '"' => {
                        self.quote = None;
                        self.token.new_sep();
                    }
                    '$' => {
                        self.token.new_var();
                    }
                    '\\' if self.in_var() => {
                        self.token.new_sep();
                        self.token.push(c);
                    }
                    c => {
                        self.token.push(c);
                    }
                }
            } else if self.in_var() {
                match c {
                    // Var token should satisfy either of:
                    // * is_ascii_alphanumeric(c)
                    // * '_'
                    c if c.is_ascii_alphanumeric() || c == '_' => {
                        self.token.push(c);
                    }
                    c if c.is_ascii_whitespace() => {
                        break;
                    }
                    '\\' => {
                        self.token.new_sep();
                    }
                    '\'' | '"' => {
                        self.quote = Some(c);
                        self.token.new_sep();
                    }
                    '$' => {
                        self.token.new_var();
                    }
                    c => {
                        self.token.new_sep();
                        self.token.push(c);
                    }
                }
            } else {
                match c {
                    c if c.is_ascii_whitespace() => {
                        break;
                    }
                    '\\' => {
                        if let Some(next) = self.chars.next() {
                            match next {
                                '\n' | '\r' | '\t' => {}
                                c => {
                                    self.token.push(c);
                                }
                            }
                            // self.token.push(next);
                            // if !next.is_ascii_whitespace() {
                            //     self.token.push(next);
                            // }
                            // c = next;
                            // continue;
                            // if next == '$' {
                            //     self.token.push('$');
                            // }
                        } else {
                            break;
                        }
                        // else {
                        //     break;
                        // }
                    }
                    '\'' | '"' => {
                        self.quote = Some(c);
                    }
                    '$' => {
                        self.token.new_var();
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
        matches!(self.token.words.last(), Some(Word::Var(_)))
    }

    fn output(&mut self) -> Token {
        self.token.reset()
    }
}
