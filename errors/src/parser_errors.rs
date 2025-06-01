use nilang_types::tokens::TokenType;

use crate::NilangErrorKind;

#[derive(Debug, Clone)]
pub enum ParserErrors {
    EndOfInput,
    UnexpectedToken(TokenType),
    ExpectedTokens(Vec<TokenType>),
    InvalidOperand,
    EmptyParenthesis,
    InvalidLiteral,
    DuplicateField(Box<str>),
}

impl std::fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sttt = match self {
            ParserErrors::EndOfInput => String::from("Unexpected end of input"),
            ParserErrors::UnexpectedToken(token) => {
                format!("Unexpected token {}", token_type_to_str(token))
            }
            ParserErrors::ExpectedTokens(tokens) => format!(
                "Expected {}",
                tokens
                    .iter()
                    .map(token_type_to_str)
                    .collect::<Vec<&str>>()
                    .join(" or ")
            ),
            ParserErrors::InvalidOperand => String::from("Invalid operand"),
            ParserErrors::EmptyParenthesis => String::from("Empty parenthesis"),
            ParserErrors::InvalidLiteral => String::from("Invalid literal"),
            ParserErrors::DuplicateField(name) => {
                format!("Duplicate field `{}`", name)
            }
        };

        write!(f, "{}", sttt)
    }
}

impl From<ParserErrors> for NilangErrorKind {
    fn from(value: ParserErrors) -> Self {
        NilangErrorKind::ParserError(value)
    }
}

impl std::error::Error for ParserErrors {}

fn token_type_to_str(token: &TokenType) -> &str {
    match token {
        TokenType::Semicolon => "semicolon",
        TokenType::Colon => "colon",
        TokenType::OpeningParenthesis => "opening parenthesis",
        TokenType::ClosingParenthesis => "closing parenthesis",
        TokenType::OpeningBrace => "opening brace",
        TokenType::ClosingBrace => "closing brace",
        TokenType::Literal(_) => "literal",
        TokenType::Operator(_) => "operator",
        TokenType::Identifier(_) => "identifier",
        TokenType::Keyword(_) => "keyword",
        TokenType::Equals => "equals",
        TokenType::Comma => "comma",
        TokenType::Dot => "dot",
    }
}
