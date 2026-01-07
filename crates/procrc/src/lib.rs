//! A tiny library to parse Procfile.

use std::io::{
    self,
    BufRead,
};

use itertools::Itertools;

/// Procfile entry, a pair of the command and its name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// A name for your command, such as web, worker and whatever.
    pub name:    String,
    /// Indicates the command that you should execute on startup, such as:
    /// - gunicorn -b :$PORT main:app
    /// - rake jobs:work
    pub cmdline: Vec<String>,
}

/// Parses Procfile.
pub fn parse<R>(r: R) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    let buf = io::BufReader::new(r);
    let lines = buf.lines().map_while(Result::ok).filter_map(|mut line| {
        // Skip comments,
        if let Some(n) = line.find('#') {
            let _comment = line.split_off(n);
        }
        // and empty lines.
        (!line.trim_ascii().is_empty()).then_some(line)
    });

    let mut entries = Vec::new();
    for line in lines {
        let (name, more) = line.split_once(':').ok_or(invalid_data("cannot find ':'"))?;
        entries.push(Entry {
            name:    name.trim_ascii().to_owned(),
            cmdline: more
                .trim_ascii()
                .split(' ')
                .filter(|t| !t.is_empty())
                .map(|t| t.to_owned())
                .collect(),
        })
    }
    Ok(entries)
}

fn invalid_data(err: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, err)
}

/// Expand tokens into a string.
#[derive(Debug)]
pub struct Expand<I> {
    tokens: Tokens<I>,
}

impl<I> Iterator for Expand<I>
where
    I: Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::Word;
        match self.tokens.next() {
            Some(Word(word)) => Some(word),
            _ => None,
        }
    }
}

/// Transforms chars to tokens.
#[derive(Debug)]
pub struct Tokens<I> {
    chars: I,
    token: String,
    quote: Option<char>,
}

/// Token string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
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

impl<I> Expand<I> {
    /// Creates a new Expand.
    pub fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Expand { tokens: Tokens::new(chars) }
    }
}

impl<I> Tokens<I> {
    /// Creates a new Tokens.
    pub fn new<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char, IntoIter = I>,
    {
        Tokens { chars: chars.into_iter(), token: String::new(), quote: None }
    }
}

impl<I> Tokens<I>
where
    I: Iterator<Item = char>,
{
    fn next(&mut self) -> Option<Token> {
        self.chars.find(|c| !c.is_ascii_whitespace()).and_then(|c| self.next_token(c))
    }

    fn next_token(&mut self, mut c: char) -> Option<Token> {
        loop {
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    break;
                }
                '\\' => {
                    if let Some(x) = self.chars.next() {
                        self.token.push(x);
                    } else {
                        break;
                    }
                }
                c => {
                    if let Some(ref quote) = self.quote {
                        if c == *quote {
                            self.quote = None;
                        } else {
                            self.token.push(c);
                        }
                    } else {
                        if c == '\'' || c == '"' {
                            self.quote = Some(c);
                        } else {
                            self.token.push(c);
                        }
                    }
                }
            }

            if let Some(x) = self.chars.next() {
                c = x;
            } else {
                break;
            }
        }
        self.output()
    }

    fn output(&mut self) -> Option<Token> {
        use Token::Word;
        if self.token.is_empty() {
            None
        } else {
            let token = self.token.clone();
            self.token = String::new();
            Some(Word(token))
        }
    }
}

/// Allow escaping.
pub fn parse_multiline<R>(r: R) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    use itertools::Itertools;
    let buf = io::BufReader::new(r);

    let mut lines = buf.lines().map_while(Result::ok).fuse();

    // let mut lines = buf.lines().map_while(Result::ok).filter_map(|mut line| {
    //     // Skip comments,
    //     if let Some(n) = line.find('#') {
    //         let _comment = line.split_off(n);
    //     }
    //     // and empty lines.
    //     if line.trim_ascii().is_empty() {
    //         return None;
    //     }
    //     line = line.trim_ascii().to_string();
    //     Some(line)
    // });

    let mut get_entry = || -> io::Result<Option<Entry>> {
        // The all lines in the chunk are collected together to form
        // a single Entry.
        let mut chunk = lines
            .by_ref()
            .filter_map(|mut line| {
                // Skip comments,
                if let Some(n) = line.find('#') {
                    let _comment = line.split_off(n);
                }
                // and empty lines.
                let trim = line.trim_ascii();
                if trim.is_empty() {
                    return None;
                }
                Some(trim.to_string())
            })
            // A '\' at the end of a line continues the line.
            //
            // TODO: use iter::take_until or something.
            // Rust stdlib may have this someday.
            // https://github.com/rust-lang/rust/issues/62208
            .take_while_inclusive(|line| line.ends_with('\\'))
            .collect::<Vec<_>>();

        if chunk.is_empty() {
            // No more entries in buf.
            return Ok(None);
        }

        // Extract the name of each entry.
        // let name = {
        //     // The first line must have ':', which separates name and entry body.
        //     let n = chunk[0].find(':').ok_or(invalid_data("cannot find ':'"))?;
        //     let mut split = chunk[0].split_off(n);
        //     assert_eq!(':', split.remove(0));
        //     std::mem::swap(&mut chunk[0], &mut split);
        //     split
        // };
        // if name.is_empty() {
        //     return Err(invalid_data("name not found"));
        // }

        // Cleanup each line.
        chunk.retain_mut(|line| {
            *line = line.trim_ascii().to_owned();
            // Remove trailing backslash.
            if line.ends_with('\\') {
                line.pop();
            }
            *line = line.trim_ascii().to_owned();
            !line.is_empty()
        });

        Ok(Some(Entry { name: "a".to_owned(), cmdline: chunk }))
    };

    let mut entries = Vec::new();
    while let Some(entry) = get_entry()? {
        entries.push(entry);
    }

    Ok(entries)
}

// fn from_flags<I, T>(args_os: I) -> Flags
// where
//     I: IntoIterator<Item = T>,
//     T: Into<String> + Clone,
// {
//     Flags::from_args_os(iter::once("procrun".to_owned()).chain(args_os.into_iter().
// map(Into::into))) }
