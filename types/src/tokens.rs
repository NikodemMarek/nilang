#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub value: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Operator,
    Identifier,
    Literal,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Equals,
    Semicolon,
    Comma,
}
