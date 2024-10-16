use nilang_types::tokens::TokenType;

#[derive(Debug, Clone)]
pub enum ParserErrors {
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
    NotANumber {
        from: (usize, usize),
        to: (usize, usize),
    },
}

impl std::fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for ParserErrors {}

impl Into<((usize, usize), (usize, usize), String)> for &ParserErrors {
    fn into(self) -> ((usize, usize), (usize, usize), String) {
        match self {
            ParserErrors::ThisNeverHappens => (
                (0, 0),
                (0, 0),
                String::from("This does not happen, what the fuck are you doing?"),
            ),
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
            ParserErrors::NotANumber { from, to } => (*from, *to, String::from("Not a number")),
        }
    }
}

fn token_type_to_str(token: &TokenType) -> &str {
    match token {
        TokenType::Semicolon => "semicolon",
        TokenType::OpeningParenthesis => "opening parenthesis",
        TokenType::ClosingParenthesis => "closing parenthesis",
        TokenType::OpeningBrace => "opening brace",
        TokenType::ClosingBrace => "closing brace",
        TokenType::Number => "number",
        TokenType::Operator => "operator",
        TokenType::Identifier => "identifier",
        TokenType::Literal => "name",
        TokenType::Equals => "equals",
        TokenType::Comma => "comma",
    }
}
