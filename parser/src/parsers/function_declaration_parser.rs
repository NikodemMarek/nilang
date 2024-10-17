use std::iter::Peekable;

use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::parse;

pub fn parse_function_declaration<'a, I>(
    tokens: &mut Peekable<I>,
    Token { end, .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    Ok(Node::FunctionDeclaration {
        name: match tokens.next() {
            Some(Token {
                token: TokenType::Identifier,
                value,
                ..
            }) => value.to_owned(),
            _ => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Identifier]),
                loc: (end.0, end.1 + 1),
            })?,
        },
        parameters: {
            if !matches!(
                tokens.next(),
                Some(Token {
                    token: TokenType::OpeningParenthesis,
                    ..
                })
            ) {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::OpeningParenthesis]),
                    loc: (end.0, end.1 + 1),
                })?;
            }

            let mut parameters = Vec::new();

            loop {
                match tokens.next() {
                    Some(Token {
                        token: TokenType::Identifier,
                        value,
                        ..
                    }) => {
                        parameters.push(value.to_owned());

                        match tokens.next() {
                            Some(Token {
                                token: TokenType::ClosingParenthesis,
                                ..
                            }) => break,
                            Some(Token {
                                token: TokenType::Comma,
                                ..
                            }) => {}
                            Some(Token { start, .. }) => Err(ParserErrors::ExpectedTokens {
                                tokens: Vec::from([
                                    TokenType::Comma,
                                    TokenType::ClosingParenthesis,
                                ]),
                                loc: *start,
                            })?,
                            None => Err(ParserErrors::EndOfInput {
                                loc: (usize::MAX, usize::MAX),
                            })?,
                        }
                    }
                    Some(Token {
                        token: TokenType::ClosingParenthesis,
                        ..
                    }) => break,
                    Some(Token { start, .. }) => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([
                            TokenType::Identifier,
                            TokenType::Literal,
                            TokenType::ClosingParenthesis,
                        ]),
                        loc: *start,
                    })?,
                    None => Err(ParserErrors::EndOfInput {
                        loc: (usize::MAX, usize::MAX),
                    })?,
                }
            }

            parameters
        },
        body: Box::new({
            if let scope @ Node::Scope(_) = parse(&mut Vec::new(), tokens)? {
                scope
            } else {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::OpeningBrace]),
                    loc: (end.0, end.1 + 1),
                })?
            }
        }),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Node,
        tokens::{Token, TokenType},
    };

    #[test]
    fn parse_function_declaration() {
        assert_eq!(
            &super::parse_function_declaration(
                &mut [
                    Token {
                        token: TokenType::Identifier,
                        value: "main".to_string(),
                        start: (0, 3),
                        end: (0, 6),
                    },
                    Token {
                        token: TokenType::OpeningParenthesis,
                        value: "(".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    },
                    Token {
                        token: TokenType::ClosingParenthesis,
                        value: ")".to_string(),
                        start: (0, 8),
                        end: (0, 8),
                    },
                    Token {
                        token: TokenType::OpeningBrace,
                        value: "{".to_string(),
                        start: (0, 9),
                        end: (0, 9),
                    },
                    Token {
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 11),
                        end: (0, 12),
                    },
                    Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 14),
                        end: (0, 14),
                    },
                    Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
                        start: (0, 15),
                        end: (0, 15),
                    },
                    Token {
                        token: TokenType::ClosingBrace,
                        value: "}".to_string(),
                        start: (0, 16),
                        end: (0, 16),
                    },
                ]
                .iter()
                .peekable(),
                &Token {
                    token: TokenType::Keyword,
                    value: "fn".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                },
            )
            .unwrap(),
            &Node::FunctionDeclaration {
                name: "main".to_string(),
                parameters: Vec::new(),
                body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                    Node::Number(6.)
                ))]))),
            }
        );
    }
}
