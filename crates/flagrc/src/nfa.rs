use super::{
    Token,
    Word,
};

struct Lexer<I> {
    bytes: I,
    state: State,
}

impl<I> Lexer<I> {
    fn new<T>(bytes: T) -> Self
    where
        T: IntoIterator<Item = u8, IntoIter = I>,
    {
        Lexer { bytes: bytes.into_iter(), state: State::FindNextNonAsciiWhiteSpace }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = u8>,
{
    fn next_token_nfa(&mut self) -> Option<Token> {
        let mut b = self.bytes.next()?;
        let mut token = Token::new();
        loop {
            let (state, action) = self.state.transition(&mut token, b);
            self.state = state;
            match action {
                Action::LeftOver => {
                    // Do not advance bytes.
                    continue;
                }
                Action::NextByte => {
                    // Keep consuming.
                }
                Action::Complete => {
                    break;
                }
            }
            if let Some(next) = self.bytes.next() {
                b = next;
            } else {
                break;
            }
        }

        if token.words.is_empty() { None } else { Some(token) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    FindNextNonAsciiWhiteSpace = 0,
    CarriageReturn = 1,
    NoQuote = 20,
    // Found "\".
    NoQuoteEscape = 21,
    // Found "$".
    NoQuoteVarStart = 22,
    NoQuoteVar = 23,
    // Found "'", but an another matching quote yet.
    InSingleQuote = 30,
    // Found '"', but an another matching quote yet.
    InDoubleQuote = 40,
    // Found "\" in double quote.
    InDoubleQuoteEscape = 41,
    // Found "$" in double quote.
    InDoubleQuoteVarStart = 42,
    InDoubleQuoteVar = 43,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    // Do not consume an input byte. Epsilon transion.
    LeftOver,
    // Consume and discard an input byte.
    NextByte,
    // Token has completed.
    Complete,
}

#[inline]
fn leftover(state: State) -> (State, Action) {
    (state, Action::LeftOver)
}

#[inline]
fn nextbyte(state: State) -> (State, Action) {
    (state, Action::NextByte)
}

#[inline]
fn complete(state: State) -> (State, Action) {
    (state, Action::Complete)
}

impl State {
    fn transition(&self, token: &mut Token, b: u8) -> (State, Action) {
        use State::*;

        match self {
            FindNextNonAsciiWhiteSpace => match b {
                b if b.is_ascii_whitespace() => nextbyte(FindNextNonAsciiWhiteSpace),
                _ => leftover(NoQuote),
            },
            CarriageReturn => match b {
                b'\n' => {
                    token.break_line(b);
                    complete(FindNextNonAsciiWhiteSpace)
                }
                _ => {
                    token.push_lit(b'\r');
                    token.push_lit(b);
                    nextbyte(NoQuote)
                }
            },
            NoQuote => match b {
                b'\r' => nextbyte(CarriageReturn),
                b'\n' => {
                    token.break_line(b);
                    complete(FindNextNonAsciiWhiteSpace)
                }
                b if b.is_ascii_whitespace() => complete(FindNextNonAsciiWhiteSpace),
                b'\\' => nextbyte(NoQuoteEscape),
                b'\'' => nextbyte(InSingleQuote),
                b'"' => nextbyte(InDoubleQuote),
                b'$' => nextbyte(NoQuoteVarStart),
                _ => {
                    token.push_lit(b);
                    nextbyte(NoQuote)
                }
            },
            NoQuoteEscape => match b {
                b'\r' => nextbyte(FindNextNonAsciiWhiteSpace),
                b'\n' => nextbyte(FindNextNonAsciiWhiteSpace),
                _ => {
                    token.push_lit(b);
                    nextbyte(NoQuote)
                }
            },
            NoQuoteVarStart => match b {
                b if b.is_ascii_alphanumeric() || b == b'_' => leftover(NoQuoteVar),
                _ => unimplemented!("Expected a variable name after $"),
            },
            NoQuoteVar => match b {
                b'\'' => nextbyte(InSingleQuote),
                b'"' => nextbyte(InDoubleQuote),
                b'\\' => nextbyte(NoQuote),
                b if b.is_ascii_whitespace() => complete(FindNextNonAsciiWhiteSpace),
                b if b.is_ascii_alphanumeric() || b == b'_' => {
                    token.push_var(b);
                    nextbyte(NoQuoteVar)
                }
                _ => {
                    token.push_lit(b);
                    nextbyte(NoQuote)
                }
            },
            InSingleQuote => match b {
                b'\'' => nextbyte(NoQuote),
                _ => {
                    token.push_lit(b);
                    nextbyte(InSingleQuote)
                }
            },
            InDoubleQuote => match b {
                b'"' => nextbyte(NoQuote),
                b'\\' => nextbyte(InDoubleQuoteEscape),
                b'$' => nextbyte(InDoubleQuoteVarStart),
                _ => {
                    token.push_lit(b);
                    nextbyte(InDoubleQuote)
                }
            },
            InDoubleQuoteEscape => match b {
                b'$' => {
                    token.push_lit(b);
                    nextbyte(InDoubleQuote)
                }
                _ => {
                    token.push_lit(b'\\');
                    token.push_lit(b);
                    nextbyte(InDoubleQuote)
                }
            },
            InDoubleQuoteVarStart => match b {
                b if b.is_ascii_alphanumeric() || b == b'_' => leftover(InDoubleQuoteVar),
                _ => unimplemented!("Expected a variable name after $"),
            },
            InDoubleQuoteVar => match b {
                b'"' => nextbyte(NoQuote),
                b'$' => nextbyte(InDoubleQuoteVarStart),
                b if b.is_ascii_alphanumeric() || b == b'_' => {
                    token.push_var(b);
                    nextbyte(InDoubleQuoteVar)
                }
                _ => {
                    token.push_lit(b);
                    nextbyte(InDoubleQuote)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::Lexer;
    use crate::Word::*;
    use crate::{
        Token,
        Word,
    };

    fn check_empty(source: &str) {
        let mut lex = Lexer::new(source.bytes());
        assert_eq!(lex.next_token_nfa(), None);
    }

    fn tokens(source: &str) -> Vec<Token> {
        let mut lex = Lexer::new(source.bytes());
        iter::from_fn(|| lex.next_token_nfa()).collect()
    }

    #[test]
    fn test_empty() {
        check_empty("");
        check_empty(" ");
        check_empty("  ");
    }

    macro_rules! Token {
        ($( $word:expr ),+ $(,)?) => {
            Token {
                words: vec![
                    $($word),+
                ]
            }
        };
    }

    fn lit(bytes: &[u8]) -> Word {
        Lit(bytes.to_vec())
    }

    fn var(bytes: &[u8]) -> Word {
        Var(bytes.to_vec())
    }

    #[test]
    fn no_tokens() {
        assert_eq!(tokens("\\"), []);
        assert_eq!(tokens("\\\n"), []);
        assert_eq!(tokens("\\\n "), []);
        assert_eq!(tokens("\n \n"), []);
        assert_eq!(tokens("\n\t\n"), []);
        assert_eq!(tokens("\n\n\n"), []);
        assert_eq!(tokens("\n\r\n"), []);
    }

    #[test]
    fn get_tokens() {
        assert_eq!(tokens("foobar"), [Token![lit(b"foobar")]]);
        assert_eq!(tokens("$foo-bar"), [Token![var(b"foo"), lit(b"-bar")]]);
        assert_eq!(tokens("A\\e"), [Token![lit(b"Ae")]]);
        assert_eq!(tokens("\"'A\\e\""), [Token![lit(b"'A\\e")]]);

        assert_eq!(tokens("foo bar"), [Token![lit(b"foo")], Token![lit(b"bar")]]);
        assert_eq!(tokens("$foo bar"), [Token![var(b"foo")], Token![lit(b"bar")]]);
        assert_eq!(
            tokens("\"foo\nbar\r\n\" baz"),
            [Token![lit(b"foo\nbar\r\n")], Token![lit(b"baz")]]
        );
    }

    #[test]
    fn get_tokens_non_ascii() {
        assert_eq!(tokens("えび"), [Token![lit("えび".as_bytes())]]);
        assert_eq!(tokens("え び"), [Token![lit("え".as_bytes())], Token![lit("び".as_bytes())]]);
        assert_eq!(tokens("え\\ び"), [Token![lit("え び".as_bytes())]]);
        assert_eq!(
            tokens("え \\ び"),
            [Token![lit("え".as_bytes())], Token![lit(" び".as_bytes())]]
        );
        assert_eq!(
            tokens("え \\\nび"),
            [Token![lit("え".as_bytes())], Token![lit("び".as_bytes())]]
        );
        assert_eq!(
            tokens("え \\\r\nび"),
            [Token![lit("え".as_bytes())], Token![lit("び".as_bytes())]]
        );
    }

    #[test]
    fn for_for_in_for() {
        // Shell prints "for" in this case.
        // https://aosabook.org/en/v1/bash.html
        assert_eq!(
            tokens("for for in for; do for=for; done; echo $for"),
            [
                Token![lit(b"for")],
                Token![lit(b"for")],
                Token![lit(b"in")],
                Token![lit(b"for;")],
                Token![lit(b"do")],
                Token![lit(b"for=for;")],
                Token![lit(b"done;")],
                Token![lit(b"echo")],
                Token![var(b"for")],
            ]
        );
    }

    #[test]
    fn line_breaks() {
        assert_eq!(
            tokens("$A-foo -x\n$B-bar -y\n$C-baz -z\n"),
            [
                Token![var(b"A"), lit(b"-foo")],
                Token![lit(b"-x"), NewLine(b'\n')],
                Token![var(b"B"), lit(b"-bar")],
                Token![lit(b"-y"), NewLine(b'\n')],
                Token![var(b"C"), lit(b"-baz")],
                Token![lit(b"-z"), NewLine(b'\n')]
            ]
        );
        assert_eq!(
            tokens("$A-foo -x\r\n$B-bar -y\r\n$C-baz -z\r\n"),
            [
                Token![var(b"A"), lit(b"-foo")],
                Token![lit(b"-x"), NewLine(b'\n')],
                Token![var(b"B"), lit(b"-bar")],
                Token![lit(b"-y"), NewLine(b'\n')],
                Token![var(b"C"), lit(b"-baz")],
                Token![lit(b"-z"), NewLine(b'\n')]
            ]
        );
    }

    #[test]
    fn escape_ascii_whitespace() {
        assert_eq!(tokens("\\ "), [Token![lit(b" ")]]);
        assert_eq!(tokens("\\ A"), [Token![lit(b" A")]]);
        assert_eq!(tokens("\\\tA"), [Token![lit(b"\tA")]]);
        assert_eq!(tokens("\\\r\nA"), [Token![lit(b"A")]]);
        assert_eq!(tokens("\\\nA"), [Token![lit(b"A")]]);
    }

    #[test]
    fn escape_in_single_quote() {
        assert_eq!(tokens("'\ne'"), [Token![lit(b"\ne")]]);
        assert_eq!(tokens("'\\\ne'"), [Token![lit(b"\\\ne")]]);
    }

    #[test]
    fn escape_in_double_quote() {
        assert_eq!(tokens("\"\\ VAR\""), [Token![lit(b"\\ VAR")]]);
        assert_eq!(tokens("\"\\$VAR\""), [Token![lit(b"$VAR")]]);
        assert_eq!(tokens("\"\\ $VAR\""), [Token![lit(b"\\ "), var(b"VAR")]]);
    }

    #[test]
    fn double_quote_no_spaces() {
        assert_eq!(tokens("A\"PPL\"E"), [Token![lit(b"APPLE")]]);
        assert_eq!(tokens("\"APPL\"E"), [Token![lit(b"APPLE")]]);
        assert_eq!(tokens("A\"PPLE\""), [Token![lit(b"APPLE")]]);
    }

    #[test]
    fn double_quote_in_single_quote() {
        assert_eq!(tokens("'A\"PPL\"E'"), [Token![lit(b"A\"PPL\"E")]]);
        assert_eq!(tokens("'\"APPL\"E'"), [Token![lit(b"\"APPL\"E")]]);
        assert_eq!(tokens("'A\"PPLE\"'"), [Token![lit(b"A\"PPLE\"")]]);
    }

    #[test]
    fn var_in_many_ways() {
        assert_eq!(tokens("\"$VAR\""), [Token![var(b"VAR")]]);
        assert_eq!(tokens("$VAR"), [Token![var(b"VAR")]]);
        assert_eq!(tokens("$VAR#a"), [Token![var(b"VAR"), lit(b"#a")]]);
        assert_eq!(tokens("$VAR-a"), [Token![var(b"VAR"), lit(b"-a")]]);
        assert_eq!(tokens("e\"$VAR\"o"), [Token![lit(b"e"), var(b"VAR"), lit(b"o")]]);
        assert_eq!(tokens("$VAR\\A"), [Token![var(b"VAR"), lit(b"A")]]);
        assert_eq!(tokens("\"$VAR\\A\""), [Token![var(b"VAR"), lit(b"\\A")]]);
        assert_eq!(tokens("$VAR\"\""), [Token![var(b"VAR")]]);
        assert_eq!(tokens("$VAR\"ABC\""), [Token![var(b"VAR"), lit(b"ABC")]]);
        assert_eq!(
            tokens("e\"$VAR\"o hello"),
            [Token![lit(b"e"), var(b"VAR"), lit(b"o")], Token![lit(b"hello")]]
        );
    }
}
