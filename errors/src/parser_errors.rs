use nilang_types::tokens::TokenType;

use crate::LexerErrors;

#[derive(Debug, Clone)]
pub enum ParserErrors {
    LexerError(LexerErrors),
    ThisNeverHappens,
    EndOfInput {
        loc: (usize, usize),
    },
    UnexpectedToken {
        token: TokenType,
        loc: (usize, usize),
    },
    ExpectedTokens {
        tokens: Vec<TokenType>,
        loc: (usize, usize),
    },
    InvalidOperand {
        loc: (usize, usize),
    },
    EmptyScope {
        from: (usize, usize),
        to: (usize, usize),
    },
    InvalidParenthesisContent {
        from: (usize, usize),
        to: (usize, usize),
    },
    EmptyParenthesis {
        from: (usize, usize),
        to: (usize, usize),
    },
    InvalidLiteral {
        from: (usize, usize),
        to: (usize, usize),
    },
    DuplicateField {
        from: (usize, usize),
        to: (usize, usize),
        name: Box<str>,
    },
}

impl std::fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for ParserErrors {}

impl From<&ParserErrors> for ((usize, usize), (usize, usize), String) {
    fn from(val: &ParserErrors) -> Self {
        match val {
            ParserErrors::ThisNeverHappens => (
                (0, 0),
                (0, 0),
                String::from("This does not happen, what the fuck are you doing?"),
            ),
            ParserErrors::LexerError(err) => err.into(),
            ParserErrors::EndOfInput { loc } => {
                (*loc, *loc, String::from("Unexpected end of input"))
            }
            ParserErrors::UnexpectedToken { token, loc } => (
                *loc,
                *loc,
                format!("Unexpected token {}", token_type_to_str(token)),
            ),
            ParserErrors::ExpectedTokens { tokens, loc } => (
                *loc,
                *loc,
                format!(
                    "Expected {}",
                    tokens
                        .iter()
                        .map(token_type_to_str)
                        .collect::<Vec<&str>>()
                        .join(" or ")
                ),
            ),
            ParserErrors::InvalidOperand { loc } => (*loc, *loc, String::from("Invalid operand")),
            ParserErrors::EmptyScope { from, to } => (*from, *to, String::from("Empty scope")),
            ParserErrors::InvalidParenthesisContent { from, to } => (
                *from,
                *to,
                String::from("Invalid content inside parenthesis"),
            ),
            ParserErrors::EmptyParenthesis { from, to } => {
                (*from, *to, String::from("Empty parenthesis"))
            }
            ParserErrors::InvalidLiteral { from, to } => {
                (*from, *to, String::from("Invalid literal"))
            }
            ParserErrors::DuplicateField { from, to, name } => {
                (*from, *to, format!("Duplicate field `{}`", name))
            }
        }
    }
}

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
