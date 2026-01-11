use super::{
    Token,
    Word,
};

struct Lexer<I> {
    bytes: I,
    token: Token,
    state: State,
}

impl<I> Lexer<I> {
    fn new<T>(bytes: T) -> Self
    where
        T: IntoIterator<Item = u8, IntoIter = I>,
    {
        Lexer {
            bytes: bytes.into_iter(),
            token: Token::new(),
            state: State::FindNextNonAsciiWhiteSpace,
        }
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
            let Transition { new_state, action, should_emit } = self.state.transition(b);

            match action {
                Action::Epsilon => (),
                Action::Discard => {
                    if let Some(next) = self.bytes.next() {
                        b = next;
                    } else {
                        break;
                    }
                }
                Action::PushLit => {
                    token.push_lit(b);
                    if let Some(next) = self.bytes.next() {
                        b = next;
                    } else {
                        break;
                    }
                }
            }

            self.state = new_state;
            if should_emit {
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
    // Consume and put an input byte to a stack.
    PushLit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Transition {
    new_state:   State,
    action:      Action,
    should_emit: bool,
}

impl Transition {
    fn new(new_state: State, action: Action, should_emit: bool) -> Self {
        Transition { new_state, action, should_emit }
    }

    fn emit(new_state: State, action: Action) -> Self {
        Transition { new_state, action, should_emit: true }
    }

    fn more(new_state: State, action: Action) -> Self {
        Transition { new_state, action, should_emit: false }
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
                b if b.is_ascii_alphanumeric() || b == b'_' => more(VarRef, PushLit),
                _ => todo!(),
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
    use super::Lexer;
    use crate::Word::*;
    use crate::{
        Token,
        Word,
    };

    #[test]
    fn test_nfa() {
        let source = "".to_owned();
        let mut lex = Lexer::new(source.bytes());
        assert_eq!(lex.next_token(), None);

        let source = " ".to_owned();
        let mut lex = Lexer::new(source.bytes());
        assert_eq!(lex.next_token(), None);

        let source = "  ".to_owned();
        let mut lex = Lexer::new(source.bytes());
        assert_eq!(lex.next_token(), None);

        let source = "foobar".to_owned();
        let mut lex = Lexer::new(source.bytes());
        assert_eq!(lex.next_token(), Some(Token { words: vec![Lit(b"foobar".to_vec())] }));
        assert_eq!(lex.next_token(), None);
    }
}
