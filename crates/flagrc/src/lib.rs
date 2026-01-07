//! A tiny library to parse a file like Procfile.

use std::io::{
    self,
    BufRead,
};
use std::iter;

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
        let tokens = Parser::new(
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
                .flat_map(|line| line.chars().chain(iter::once('\n'))),
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
pub struct Parser<I>
where
    I: Iterator,
{
    lex: Tokens<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    /// Creates a new Tokens.
    ///
    /// ```
    /// # use flagrc::Parser;
    /// let mut tokens = Parser::new("foo\"ba\"r baz".chars());
    /// assert_eq!(tokens.next().unwrap(), "foobar");
    /// assert_eq!(tokens.next().unwrap(), "baz");
    /// assert_eq!(tokens.next(), None);
    /// ```
    pub fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Parser { lex: Tokens::new(chars) }
    }
}

impl<I> Iterator for Parser<I>
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

struct Tokens<I>
where
    I: Iterator,
{
    chars: iter::Peekable<I>,
    token: String,
    quote: Option<char>,
    brace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Word(String),
    Env { key: String, value: String },
}

impl<I> Tokens<I>
where
    I: Iterator,
{
    fn new<T>(chars: T) -> Self
    where
        I: Iterator<Item = char>,
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens {
            chars: chars.into_iter().peekable(),
            token: String::new(),
            quote: None,
            brace: false,
        }
    }
}

impl<I> Tokens<I>
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
                            // dbg!(_skipped);
                        } else {
                            break;
                        }
                    }
                    '\'' | '"' => {
                        self.quote = Some(c);
                    }
                    '$' => {
                        if let Some(peek) = self.chars.peek()
                            && *peek == '{'
                        {
                            dbg!("this value need to be expanded");
                        }
                        self.token.push(c);
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
