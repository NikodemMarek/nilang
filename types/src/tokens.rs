use crate::nodes::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Operator(Operator),
    Identifier(Box<str>),
    Keyword(Box<str>),
    Literal(Box<str>),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Equals,
    Semicolon,
    Comma,
}
