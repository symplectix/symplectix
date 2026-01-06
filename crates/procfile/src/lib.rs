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
    let mut lines = buf.lines().map_while(Result::ok).filter_map(|mut line| {
        // Skip comments,
        if let Some(n) = line.find('#') {
            let _comment = line.split_off(n);
        }
        // and empty lines.
        (!line.trim_ascii().is_empty()).then_some(line)
    });

    let mut get_entry = || -> io::Result<Option<Entry>> {
        // The all lines in the chunk are collected together to form
        // a single Entry.
        let mut chunk = lines
            .by_ref()
            // Rust stdlib may have this someday.
            // https://github.com/rust-lang/rust/issues/62208
            .take_while_inclusive(|line| line.trim_ascii().ends_with('\\'))
            .collect::<Vec<_>>();
        if chunk.is_empty() {
            return Ok(None);
        }

        // Extract the name of each entry.
        let name = {
            // The first line must have ':', which separates name and entry body.
            let n = chunk[0].find(':').ok_or(io::Error::other("cannot find ':'"))?;
            let mut split = chunk[0].split_off(n);
            assert_eq!(':', split.remove(0));
            std::mem::swap(&mut chunk[0], &mut split);
            split
        };
        if name.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "name not found"));
        }

        // Cleanup each line.
        chunk.retain_mut(|line| {
            // line.retain(|c| c != ':');
            *line = line.trim_ascii().to_owned();
            while line.ends_with('\\') || line.ends_with(' ') {
                line.pop();
            }
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

    use super::parse;

    #[test]
    fn check_parse() {
        let procfile = r#"
# comment
  # comment
    # comment
0:
1: gunicorn -b :$PORT main:app
2: abc\
--kill-after 10s --env APPLE=banana \
-- foo a b c
3: --backslask C:\path \
foo a=1 b=2 c=3
4: --kill-after 10m --timeout-is-ok -- foo a 1 b 2 c 3
5: a b c
6: a \
\
b \ c \ d e
7: \
# aaa
--env FOO=foo \ # env
# skip --env BAR=bar \
a \
b \
c
# :
# foo a b c
# : foo a b c
"#;
        let entries = parse(io::Cursor::new(procfile)).unwrap();
        println!("{entries:?}");
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

    //     #[test]
    //     fn check() {
    //         let procfile = r#"
    // # comment
    //   # comment
    //     # comment
    // xxx(aaa): foo a b c
    // xxx(aaa): --kill-after 10s --env APPLE=banana -- foo a b c
    // yyy(bbb): --wait /tmp/aaa --on-exit /tmp/bbb foo a=1 b=2 c=3
    // zzz(ccc): --kill-after 10m --timeout-is-ok -- foo a 1 b 2 c 3
    //         "#;

    //         let parsed = parse(io::Cursor::new(procfile)).unwrap();
    //         assert_eq!(parsed.len(), 4);

    //         assert_eq!(parsed[0].flags[0], "foo");
    //         assert_eq!(parsed[0].flags[1..], ["a", "b", "c"]);
    //         assert!(parsed[0].timeout.kill_after.is_none());

    //         assert_eq!(parsed[1].program, "foo");
    //         assert_eq!(parsed[1].args, ["a", "b", "c"]);
    //         assert_eq!(parsed[1].envs, ["APPLE=banana"]);
    //         assert!(parsed[1].timeout.kill_after.is_some());

    //         assert_eq!(parsed[2].program, "foo");
    //         assert_eq!(parsed[2].args, ["a=1", "b=2", "c=3"]);
    //         assert_eq!(parsed[2].hook.wait_for, [Path::new("/tmp/aaa")]);
    //         assert_eq!(parsed[2].hook.on_exit.as_ref().unwrap(), Path::new("/tmp/bbb"));

    //         assert_eq!(parsed[3].program, "foo");
    //         assert_eq!(parsed[3].args, ["a", "1", "b", "2", "c", "3"]);
    //         assert!(parsed[3].timeout.kill_after.is_some());
    //         assert!(parsed[3].timeout.is_ok);

    //         for flags in parsed {
    //             println!("{flags:?}");
    //         }
    // }
}
