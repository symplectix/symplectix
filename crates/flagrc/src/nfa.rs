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
            let Transition { state, action, emit } = self.state.transition(b);
            self.state = state;
            match action {
                Action::Epsilon => {
                    // do not advance self.bytes
                    continue;
                }
                Action::Discard => {
                    // discarding b
                }
                Action::PushLit => {
                    token.push_lit(b);
                }
                Action::PushVar => {
                    token.push_var(b);
                }
                Action::LineBreak => {
                    token.break_line(b);
                    break;
                }
            }
            if emit {
                break;
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
    Literal = 1,
    // Found "\".
    InEscape = 2,
    // Found "$".
    VarStart,
    Var,
    // Found "'", but an another matching quote yet.
    InSingleQuote,
    // Found '"', but an another matching quote yet.
    InDoubleQuote,
    // Found "$" in double quote.
    DoubleQuotingVarStart,
    DoubleQuotingVar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    // Do not consume an input byte.
    Epsilon,
    // Consume and discard an input byte.
    Discard,
    // Found the *non-escaped* newline delimiter.
    LineBreak,
    // Consume and push an input byte as Word::Lit.
    PushLit,
    // Consume and push an input byte as Word::Var.
    PushVar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Transition {
    state:  State,
    action: Action,
    emit:   bool,
}

impl Transition {
    fn new(state: State, action: Action, emit: bool) -> Self {
        Transition { state, action, emit }
    }

    fn emit(state: State, action: Action) -> Self {
        Transition { state, action, emit: true }
    }

    fn more(state: State, action: Action) -> Self {
        Transition { state, action, emit: false }
    }
}

impl State {
    fn transition(&self, b: u8) -> Transition {
        use Action::*;
        use State::*;
        let emit = Transition::emit;
        let more = Transition::more;

        match self {
            FindNextNonAsciiWhiteSpace => match b {
                b if b.is_ascii_whitespace() => more(FindNextNonAsciiWhiteSpace, Discard),
                _ => more(Literal, Epsilon),
            },
            Literal => match b {
                b if b.is_ascii_whitespace() => {
                    if b == b'\n' {
                        emit(FindNextNonAsciiWhiteSpace, LineBreak)
                    } else {
                        emit(FindNextNonAsciiWhiteSpace, Discard)
                    }
                }
                b'\\' => more(InEscape, Discard),
                b'\'' => more(InSingleQuote, Discard),
                b'"' => more(InDoubleQuote, Discard),
                b'$' => more(VarStart, Discard),
                _ => more(Literal, PushLit),
            },
            InEscape => match b {
                b'\r' => more(FindNextNonAsciiWhiteSpace, Discard),
                b'\n' => more(FindNextNonAsciiWhiteSpace, Discard),
                _ => more(Literal, PushLit),
            },
            InSingleQuote => match b {
                b'\'' => more(Literal, Discard),
                _ => more(InSingleQuote, PushLit),
            },
            InDoubleQuote => match b {
                b'"' => more(Literal, Discard),
                b'$' => more(DoubleQuotingVarStart, Discard),
                _ => more(InDoubleQuote, PushLit),
            },
            DoubleQuotingVarStart => match b {
                b if b.is_ascii_alphanumeric() || b == b'_' => more(DoubleQuotingVar, Epsilon),
                _ => unimplemented!("Expected a variable name after $"),
            },
            DoubleQuotingVar => match b {
                b'"' => more(Literal, Discard),
                b'$' => more(DoubleQuotingVarStart, Discard),
                b if b.is_ascii_alphanumeric() || b == b'_' => more(DoubleQuotingVar, PushVar),
                _ => more(InDoubleQuote, PushLit),
            },
            VarStart => match b {
                b if b.is_ascii_alphanumeric() || b == b'_' => more(Var, Epsilon),
                _ => unimplemented!("Expected a variable name after $"),
            },
            Var => match b {
                b'\'' => emit(InSingleQuote, Discard),
                b'"' => emit(InDoubleQuote, Discard),
                b if b.is_ascii_whitespace() => emit(FindNextNonAsciiWhiteSpace, Discard),
                b if b.is_ascii_alphanumeric() || b == b'_' => more(Var, PushVar),
                _ => more(Literal, PushLit),
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
    fn test_nfa() {
        assert_eq!(tokens("foobar"), vec![Token![lit(b"foobar")]]);
        assert_eq!(tokens("$foo-bar"), vec![Token![var(b"foo"), lit(b"-bar")]]);
        assert_eq!(tokens("foo bar"), vec![Token![lit(b"foo")], Token![lit(b"bar")]]);
        assert_eq!(tokens("$foo bar"), vec![Token![var(b"foo")], Token![lit(b"bar")]]);
    }

    #[test]
    fn no_tokens() {
        assert_eq!(tokens("\\"), vec![]);
        assert_eq!(tokens("\\\n"), vec![]);
        assert_eq!(tokens("\\\n "), vec![]);
    }

    #[test]
    fn escape_ascii_whitespace() {
        assert_eq!(tokens("\\ "), vec![Token![lit(b" ")]]);
        assert_eq!(tokens("\\ A"), vec![Token![lit(b" A")]]);
        assert_eq!(tokens("\\\tA"), vec![Token![lit(b"\tA")]]);

        assert_eq!(tokens("\\\r\nA"), vec![Token![lit(b"A")]]);
        assert_eq!(tokens("\\\nA"), vec![Token![lit(b"A")]]);
    }

    #[test]
    fn line_breaks() {
        assert_eq!(
            tokens("$A-foo -x\n$B-bar -y\n$C-baz -z\n"),
            vec![
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
    fn no_line_breaks() {
        assert_eq!(tokens("\n \n"), vec![]);
        assert_eq!(tokens("\n\t\n"), vec![]);
        assert_eq!(tokens("\n\n\n"), vec![]);
        assert_eq!(tokens("\n\r\n"), vec![]);
    }

    #[test]
    fn escape_in_single_quote() {
        assert_eq!(tokens("'\ne'"), vec![Token![lit(b"\ne")]]);
    }

    #[test]
    fn double_quote_no_spaces() {
        assert_eq!(tokens("A\"PPL\"E"), vec![Token![lit(b"APPLE")]]);
        assert_eq!(tokens("\"APPL\"E"), vec![Token![lit(b"APPLE")]]);
        assert_eq!(tokens("A\"PPLE\""), vec![Token![lit(b"APPLE")]]);
    }

    #[test]
    fn double_quote_in_single_quote() {
        assert_eq!(tokens("'A\"PPL\"E'"), vec![Token![lit(b"A\"PPL\"E")]]);
        assert_eq!(tokens("'\"APPL\"E'"), vec![Token![lit(b"\"APPL\"E")]]);
        assert_eq!(tokens("'A\"PPLE\"'"), vec![Token![lit(b"A\"PPLE\"")]]);
    }
}
