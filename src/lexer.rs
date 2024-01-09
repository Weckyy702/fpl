use std::num::ParseFloatError;
use thiserror::Error;

use crate::token::Token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Unexpected character {0}")]
    UnexpectedChar(char),
    #[error("Invalid numeric literal {0:?}: {1}")]
    InvalidNumber(String, ParseFloatError),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Lexer<I> {
    it: I,
    next: Option<char>,
}

fn is_separator(c: char) -> bool {
    matches!(c, ':' | '(' | ')' | ';') || c.is_ascii_whitespace()
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new<It>(it: It) -> Self
    where
        It: IntoIterator<IntoIter = I>,
    {
        let mut it = it.into_iter();
        let next = it.next();
        Self { it, next }
    }

    fn is_done(&self) -> bool {
        self.next.is_none()
    }

    fn peek(&self) -> Option<char> {
        self.next
    }

    fn peek_or_eof(&self) -> Result<char> {
        self.next.ok_or(Error::UnexpectedEOF.into())
    }

    fn consume(&mut self) {
        self.next = self.it.next();
    }

    fn skip(&mut self) {
        let mut in_comment = false;
        while let Some(c) = self.peek() {
            match c {
                '\n' => in_comment = false,
                '#' => in_comment = true,
                _ if in_comment => (),
                c if c.is_whitespace() => (),
                _ => return,
            }
            self.consume();
        }
    }

    fn read_identifier(&mut self, start: Option<char>) -> Result<Token> {
        let mut s = String::with_capacity(16);

        if let Some(c) = start {
            s.push(c);
        }

        while let Some(c) = self.peek() {
            if is_separator(c) {
                break;
            }
            self.consume();
            s.push(c);
        }

        if s.starts_with(|c: char| c.is_ascii_digit() || c == '-') {
            return s
                .parse()
                .map(Token::Number)
                .map_err(|e| Error::InvalidNumber(s, e));
        }

        Ok(Token::Identifier(s))
    }

    fn next_token(&mut self) -> Result<Token> {
        macro_rules! ok {
            ($kind:ident) => {{
                self.consume();
                return Ok(Token::$kind);
            }};
            ($e:expr) => {{
                return Ok($e);
            }};
        }

        match self.peek_or_eof()? {
            ':' => ok!(Colon),
            '(' => ok!(LParen),
            ')' => ok!(RParen),
            ';' => ok!(Semi),
            '-' => {
                self.consume();
                if let Some('>') = self.peek() {
                    ok!(ThinArrow)
                } else {
                    return self.read_identifier(Some('-'));
                }
            }
            _ => self.read_identifier(None),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip();

        if self.is_done() {
            return None;
        }

        Some(self.next_token())
    }
}
