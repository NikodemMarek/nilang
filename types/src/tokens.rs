use crate::nodes::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Function,
    Variable,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Literal(Box<str>),
    Identifier(Box<str>),
    Keyword(Keyword),
    Operator(Operator),
    Equals,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Semicolon,
}
