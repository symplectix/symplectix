//! A tiny library to parse Procfile.

use std::io::{
    self,
    BufRead,
};

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

#[cfg(test)]
fn parse_multiline<R>(r: R) -> io::Result<Vec<Entry>>
where
    R: io::Read,
{
    use itertools::Itertools;
    let buf = io::BufReader::new(r);
    let mut lines = buf.lines().map_while(Result::ok).filter_map(|mut line| {
        // Skip comments,
        if let Some(n) = line.find('#') {
            let _comment = line.split_off(n);
        }
        // and empty lines.
        if line.trim_ascii().is_empty() {
            return None;
        }
        line = line.trim_ascii().to_string();
        Some(line)
    });

    let mut get_entry = || -> io::Result<Option<Entry>> {
        // The all lines in the chunk are collected together to form
        // a single Entry.
        let mut chunk = lines
            .by_ref()
            // Rust stdlib may have this someday.
            // https://github.com/rust-lang/rust/issues/62208
            .take_while_inclusive(|line| line.ends_with('\\'))
            .collect::<Vec<_>>();

        if chunk.is_empty() {
            // No more lines.
            return Ok(None);
        }

        // Extract the name of each entry.
        let name = {
            // The first line must have ':', which separates name and entry body.
            let n = chunk[0].find(':').ok_or(invalid_data("cannot find ':'"))?;
            let mut split = chunk[0].split_off(n);
            assert_eq!(':', split.remove(0));
            std::mem::swap(&mut chunk[0], &mut split);
            split
        };
        if name.is_empty() {
            return Err(invalid_data("name not found"));
        }

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

        Ok(Some(Entry { name, cmdline: chunk }))
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

#[cfg(test)]
mod tests {
    use std::io;

    use super::{
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
s2: abc --kill-after 10s --env APPLE=banana -- foo a b c
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

m4: --kill-after 10m --timeout-is-ok -- \
foo a 1 b 2 c 3

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
}
