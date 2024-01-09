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
