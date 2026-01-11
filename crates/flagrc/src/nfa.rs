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
    fn next_token(&mut self) -> Option<Token> {
        let mut b = self.bytes.next()?;
        let mut token: Token = Token::new();
        loop {
            let Transition { state: new_state, action, emit: should_emit } =
                self.state.transition(b);
            self.state = new_state;
            if should_emit {
                break;
            }

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

    // Just after '\'.
    Escaping = 2,
    // A $VARIABLE like this
    BeginVar,
    VarRef,
    // Found "'", but an another matching quote yet.
    SingleQuoting,
    // Found '"', but an another matching quote yet.
    DoubleQuoting,
    DoubleQuotingBeginVar,
    DoubleQuotingVarRef,
    // Look before the *non-escaped* newline delimiter.
    LineBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    // Do not consume an input byte.
    Epsilon,
    // Consume and discard an input byte.
    Discard,
    // Consume and push an input byte as a literal.
    PushLit,
    // Consume and push an input byte as a variable.
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
                b if b.is_ascii_whitespace() => emit(FindNextNonAsciiWhiteSpace, Discard),
                b'\\' => more(Escaping, Discard),
                b'\'' => more(SingleQuoting, Discard),
                b'"' => more(DoubleQuoting, Discard),
                b'$' => more(BeginVar, Discard),
                _ => more(Literal, PushLit),
            },
            Escaping => match b {
                b'\r' => todo!(),
                b'\n' => more(Literal, Discard),
                _ => more(Literal, PushLit),
            },
            SingleQuoting => match b {
                b'\'' => more(Literal, Discard),
                _ => more(SingleQuoting, PushLit),
            },
            DoubleQuoting => match b {
                b'"' => more(Literal, Discard),
                b'$' => more(DoubleQuotingBeginVar, Discard),
                _ => more(DoubleQuoting, PushLit),
            },
            DoubleQuotingBeginVar => match b {
                _ => todo!(),
                _ => todo!(),
            },
            DoubleQuotingVarRef => match b {
                _ => todo!(),
                _ => todo!(),
            },
            BeginVar => match b {
                b if b.is_ascii_alphanumeric() || b == b'_' => more(VarRef, Epsilon),
                _ => unimplemented!("Expected a variable name after $"),
            },
            VarRef => match b {
                b'\'' => emit(SingleQuoting, Discard),
                b'"' => emit(DoubleQuoting, Discard),
                b if b.is_ascii_whitespace() => emit(FindNextNonAsciiWhiteSpace, Discard),
                b if b.is_ascii_alphanumeric() || b == b'_' => more(VarRef, PushVar),
                _ => more(Literal, PushLit),
            },
            LineBreak => match b {
                //
                _ => todo!(),
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
        assert_eq!(lex.next_token(), None);
    }

    fn tokens(source: &str) -> Vec<Token> {
        let mut lex = Lexer::new(source.bytes());
        iter::from_fn(|| lex.next_token()).collect()
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
    fn whitespace_should_work() {
        assert_eq!(tokens("\\ "), vec![Token![lit(b" ")]]);
        assert_eq!(tokens("\\ e"), vec![Token![lit(b" e")]]);
        assert_eq!(tokens("'\ne'"), vec![Token![lit(b"\ne")]]);
    }
}
