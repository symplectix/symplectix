//! A tiny library to parse Procfile.

use std::io::{
    self,
    BufRead,
};
use std::iter;

use itertools::Itertools;

/// Procrc entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Indicates the command that you should execute on startup, such as:
    /// - gunicorn -b :$PORT main:app
    /// - rake jobs:work
    pub cmdline: Vec<String>,
}

/// Parses procrc.
pub fn parse<R>(r: R) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    let rc = {
        let mut vec = Vec::new();
        let buf = io::BufReader::new(r);
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
                // nb. Each string produced by buf.lines() will not have a newline
                // byte (the `0xA` byte) or `CRLF` (`0xD`, `0xA` bytes) at the end.
                .flat_map(|line| line.chars().chain(iter::once('\n'))),
        )
        .collect::<Vec<_>>();

        if tokens.is_empty() { None } else { Some(Entry { cmdline: tokens }) }
    };
    Ok(iter::from_fn(get_entry).collect())
}

/// Expand tokens into a string.
#[derive(Debug)]
pub struct Tokens<I> {
    lex: Lexer<I>,
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::Word;
        match self.lex.next() {
            Some(Word(word)) => Some(word),
            _ => None,
        }
    }
}

/// Transforms an input chars into a sequence of tokens.
#[derive(Debug)]
struct Lexer<I> {
    chars: I,
    token: String,
    quote: Option<char>,
}

/// Token string.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// Just a word.
    Word(String),
    /// Need to expand.
    Env {
        /// Key of env.
        key:   String,
        /// Value of env.
        value: String,
    },
}

impl<I> Tokens<I> {
    /// Creates a new Expand.
    pub fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens { lex: Lexer::new(chars) }
    }
}

impl<I> Lexer<I> {
    /// Creates a new Tokens.
    fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Lexer { chars: chars.into_iter(), token: String::new(), quote: None }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn next(&mut self) -> Option<Token> {
        self.chars.find(|c| !c.is_ascii_whitespace()).map(|c| self.next_token(c))
    }

    fn next_token(&mut self, mut c: char) -> Token {
        loop {
            if let Some(ref quote) = self.quote {
                if c == *quote {
                    self.quote = None;
                } else {
                    self.token.push(c);
                }
            } else {
                match c {
                    c if c.is_ascii_whitespace() => {
                        break;
                    }
                    '\\' => {
                        if let Some(_skipped) = self.chars.next() {
                            // dbg!(skipped);
                        } else {
                            break;
                        }
                    }
                    '\'' | '"' => {
                        self.quote = Some(c);
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

    fn output(&mut self) -> Token {
        use Token::Word;
        let token = self.token.clone();
        self.token = String::new();
        Word(token)
    }
}
