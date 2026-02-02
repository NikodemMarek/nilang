use std::fmt::Debug;

use crate::nodes::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Clone, PartialEq)]
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
    Dot,
    Semicolon,
    Colon,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenType::")?;
        match self {
            TokenType::Literal(value) => write!(f, "Literal({:?}.into())", value),
            TokenType::Identifier(name) => write!(f, "Identifier({:?}.into())", name),
            TokenType::Keyword(keyword) => write!(f, "Keyword({:?})", keyword),
            TokenType::Operator(operator) => write!(f, "Operator({:?})", operator),
            TokenType::Equals => write!(f, "Equals"),
            TokenType::OpeningParenthesis => write!(f, "OpeningParenthesis"),
            TokenType::ClosingParenthesis => write!(f, "ClosingParenthesis"),
            TokenType::OpeningBrace => write!(f, "OpeningBrace"),
            TokenType::ClosingBrace => write!(f, "ClosingBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Keyword {
    Function,
    Variable,
    Return,
    Structure,
    If,
    ElseIf,
    Else,
}

impl Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keyword::")?;
        match self {
            Keyword::Function => write!(f, "Function"),
            Keyword::Variable => write!(f, "Variable"),
            Keyword::Return => write!(f, "Return"),
            Keyword::Structure => write!(f, "Structure"),
            Keyword::If => write!(f, "If"),
            Keyword::ElseIf => write!(f, "ElseIf"),
            Keyword::Else => write!(f, "Else"),
        }
    }
}
