use std::fmt::{Display, Write};

use crate::Identifier;

#[derive(Debug)]
pub enum Token {
    Colon,
    ThinArrow,
    LParen,
    RParen,
    Semi,
    Number(f64),
    Identifier(Identifier),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Colon => f.write_char(':'),
            ThinArrow => f.write_str("=>"),
            LParen => f.write_char('('),
            RParen => f.write_char(')'),
            Semi => f.write_char(';'),
            Number(x) => write!(f, "{x}"),
            Identifier(name) => write!(f, "{name}"),
        }
    }
}
