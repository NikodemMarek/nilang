use crate::nodes::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Literal(Box<str>),
    Identifier(Box<str>),
    Keyword(Box<str>),
    Operator(Operator),
    Equals,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Semicolon,
}
