//! A tiny library to parse a file like Procfile.

use std::any::Any;
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
    let rcfile = {
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

    let rc_clone = rcfile.clone();
    let rc_clone_iter = rc_clone
        .iter()
        // nb. Each string produced by buf.lines() will not have a
        // newline byte (the `0xA` byte) or `CRLF` at the end.
        .flat_map(|line| line.trim_ascii().chars().chain(iter::once('\n')));
    let new_tokens = Tokens::new(rc_clone_iter, envs);
    Ok(new_tokens.collect())

    // Tokens emits `Some("")` when input is eg. "\\ ".
    // This behavior is important in the current implementation:
    // if input is something like "\\ x", returning None will not output x.
    // .filter(|t| !t.is_empty())
    // .collect::<Vec<_>>()

    // let mut rc_lines = rcfile.iter();
    // let get_entry = || -> Option<Entry> {
    //     let tokens = Tokens::new(
    //         // The all lines in the chunk are collected together to form
    //         // a single Entry.
    //         rc_lines
    //             .by_ref()
    //             // A '\' at the end of a line continues the line.
    //             //
    //             // TODO: Use iter::take_until or something.
    //             // Rust stdlib may have this someday.
    //             // https://github.com/rust-lang/rust/issues/62208
    //             .take_while_inclusive(|line| line.trim_ascii().ends_with('\\'))
    //             // nb. Each string produced by buf.lines() will not have a
    //             // newline byte (the `0xA` byte) or `CRLF` at the end.
    //             .flat_map(|line| line.trim_ascii().chars().chain(iter::once('\n'))),
    //         envs.as_ref(),
    //     )
    //     // Tokens emits `Some("")` when input is eg. "\\ ".
    //     // This behavior is important in the current implementation:
    //     // if input is something like "\\ x", returning None will not output x.
    //     .filter(|t| !t.is_empty())
    //     .collect::<Vec<_>>();

    //     if tokens.is_empty() { None } else { Some(Entry { flag: tokens }) }
    // };
    // Ok(iter::from_fn(get_entry).collect())
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
    /// assert_eq!(tokens.next().unwrap(), "foobar");
    /// assert_eq!(tokens.next().unwrap(), "baz");
    /// assert_eq!(tokens.next(), None);
    /// ```
    pub fn new<T>(chars: T, envs: Option<HashMap<String, String>>) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens { lex: Lexer::new(chars), envs: envs.unwrap_or_else(HashMap::new) }
    }

    fn get_all(&mut self) -> Option<Token>
    where
        I: Iterator<Item = char>,
    {
        self.lex.next()
        // let mut vec = Vec::new();
        // for token in self.lex.by_ref() {
        //     for word in token.words {
        //         // if matches!(word, Word::TokenSeparator) {
        //         //     return Some(Token { words: vec });
        //         // }
        //         // vec.push(word);
        //         match word {
        //             Word::TokenSeparator => {
        //                 return Some(Token { words: vec });
        //                 // vec.push(out.clone());
        //                 // dbg!(&vec);
        //                 // mem::replace(&mut out, String::new());
        //             }
        //             Word::Split => {
        //                 vec.push(word);
        //             }
        //             Word::Lit(_) => {
        //                 vec.push(word);
        //             }
        //             Word::Var(ref var) => {
        //                 // vec.push(word);
        //                 if let Some(val) = self.envs.as_ref().and_then(|es| es.get(var.as_str()))
        // {                     vec.push(Word::Var(val.clone()));
        //                 }
        //             }
        //         }
        //     }
        // }
        // None
    }
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = char>,
{
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex.next().map(|token| {
            Entry {
                flag: token
                    .words
                    .into_iter()
                    .filter_map(|word| match word {
                        // Word::TokenSeparator => {
                        //     // vec.push(out.clone());
                        //     // dbg!(&vec);
                        //     // mem::replace(&mut out, String::new());
                        // }
                        Word::Split => None,
                        Word::Lit(lit) => Some(lit),
                        Word::Var(var) => {
                            if let Some(val) = self.envs.get(var.as_str()) {
                                Some(val.to_owned())
                            } else {
                                Some(var)
                            }
                        }
                    })
                    .collect(),
            }
        })
    }

    //     for word in token.words.iter_mut() {
    //         todo!()
    //         // match word {
    //         //     // Word::TokenSeparator => {
    //         //     //     // vec.push(out.clone());
    //         //     //     // dbg!(&vec);
    //         //     //     // mem::replace(&mut out, String::new());
    //         //     // }
    //         //     Word::Split => None,
    //         //     Word::Lit(lit) => Some(lit),
    //         //     Word::Var(var) => {
    //         //         Some(var)
    //         //         // if let Some(val) = self.envs.as_ref().and_then(|es|
    //         // es.get(var.as_str()))         // {     out.push_str(val);
    //         //         // }
    //         //     }
    //         // }
    //     }
    //     token
    // })
}

struct Lexer<I> {
    chars: I,
    token: Token,
    quote: Option<char>,
}

// Tokens are represented as a list of Word.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Word {
    // Use this marker when you want to push a new word,
    // but cannot at the moment. e.g, when there are
    // consecutive Vars.
    Split,
    // A literal string.
    Lit(String),
    // $LIKE_THIS
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
        if let Some(last) = self.words.last_mut() {
            match last {
                Word::Split => self.words.push(Word::Lit(c.to_string())),
                Word::Lit(s) => s.push(c),
                Word::Var(s) => s.push(c),
            }
        } else {
            self.words.push(Word::Lit(c.to_string()));
        }
    }

    fn split(&mut self) {
        if let Some(last) = self.words.last() {
            if !(matches!(last, Word::Split)) {
                self.words.push(Word::Split);
            }
        } else {
            self.words.push(Word::Split);
        }
    }

    fn begin_var(&mut self) {
        self.words.push(Word::Var(String::new()))
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
                        // break;
                        self.token.split();
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
                        // self.words.words.push(Word::TokenSeparator);
                        break;
                    }
                    c if c.is_ascii_whitespace() => {
                        // break;
                        self.token.split();
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
