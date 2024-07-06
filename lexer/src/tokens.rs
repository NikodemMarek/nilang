#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Operator,
    Keyword,
    Literal,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
}
