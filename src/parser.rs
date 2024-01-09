use std::mem;
use thiserror::Error;

use crate::{ast::untyped::ASTNode, token::Token};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected end of token stream")]
    UnexpectedEnd,
    #[error("Unmatched parantheses")]
    UnmatchedParen,
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
    #[error("Unclosed paranthesis")]
    UnclosedParen,
}

type Result<T> = std::result::Result<T, Error>;

pub struct Parser<I> {
    it: I,
    next: Option<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
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

    fn peek(&mut self) -> Option<&Token> {
        self.next.as_ref()
    }

    fn peek_or_eof(&mut self) -> Result<&Token> {
        self.peek().ok_or(Error::UnexpectedEnd.into())
    }

    fn consume(&mut self) -> Result<Token> {
        let cur = mem::replace(&mut self.next, self.it.next());
        cur.ok_or(Error::UnexpectedEnd.into())
    }

    //Atom: IDENT | Number | ';'
    fn parse_atom(&mut self) -> Result<ASTNode> {
        use Token as T;

        match self.consume()? {
            T::Identifier(name) => Ok(ASTNode::atom(name)),
            T::Number(x) => Ok(ASTNode::number(x)),
            T::Semi => Ok(ASTNode::empty()),
            t => return Err(Error::UnexpectedToken(t)),
        }
    }

    fn parse_sublevel_expression(&mut self) -> Result<ASTNode> {
        use Token as T;
        match self.peek_or_eof()? {
            T::LParen => {
                self.consume()?;
                let node = self.parse_toplevel_expression()?;
                let Ok(T::RParen) = self.consume() else {
                    return Err(Error::UnclosedParen);
                };
                Ok(node)
            }
            T::RParen => Ok(ASTNode::empty()),
            _ => self.parse_atom(),
        }
    }

    fn parse_toplevel_expression(&mut self) -> Result<ASTNode> {
        use Token as T;

        match self.consume()? {
            T::Identifier(name) => {
                let mut node = ASTNode::atom(name);
                loop {
                    let arg = self.parse_sublevel_expression()?;
                    if arg.is_empty() {
                        break;
                    }
                    node = ASTNode::call(node, arg);
                }
                Ok(node)
            }
            t => Err(Error::UnexpectedToken(t)),
        }
    }

    fn parse_next(&mut self) -> Result<ASTNode> {
        self.parse_toplevel_expression()
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<ASTNode>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }

        Some(self.parse_next())
    }
}
