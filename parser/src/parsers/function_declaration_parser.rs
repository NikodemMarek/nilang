use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::parse;

pub fn parse_function_declaration<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Keyword,
            value,
            ..
        })) => {
            if value != "fn" {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::Keyword]),
                    loc: (0, 1),
                })?
            }
        }
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Keyword]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let name = match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Identifier,
            value,
            ..
        })) => value.to_owned(),
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Identifier]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::OpeningParenthesis,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::OpeningParenthesis]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let parameters = {
        let mut parameters = Vec::new();

        loop {
            match tokens.next() {
                Some(Ok(Token {
                    token: TokenType::Identifier,
                    value,
                    ..
                })) => {
                    parameters.push(value.to_owned());

                    match tokens.next() {
                        Some(Ok(Token {
                            token: TokenType::ClosingParenthesis,
                            ..
                        })) => break,
                        Some(Ok(Token {
                            token: TokenType::Comma,
                            ..
                        })) => {}
                        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
                            tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                            loc: start,
                        })?,
                        Some(_) | None => Err(ParserErrors::EndOfInput {
                            loc: (usize::MAX, usize::MAX),
                        })?,
                    }
                }
                Some(Ok(Token {
                    token: TokenType::ClosingParenthesis,
                    ..
                })) => break,
                Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::Identifier, TokenType::ClosingParenthesis]),
                    loc: start,
                })?,
                Some(_) | None => Err(ParserErrors::EndOfInput {
                    loc: (usize::MAX, usize::MAX),
                })?,
            }
        }

        parameters
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::OpeningBrace,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::OpeningBrace]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let body = {
        let mut program = Vec::new();

        loop {
            match tokens.peek() {
                Some(Ok(Token {
                    token: TokenType::ClosingBrace,
                    ..
                })) => {
                    tokens.next();
                    break;
                }
                Some(Ok(Token { .. })) => {
                    program.push(parse(tokens)?);
                }
                Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
                None => Err(ParserErrors::EndOfInput {
                    loc: (usize::MAX, usize::MAX),
                })?,
            }
        }

        program
    };

    Ok(Node::FunctionDeclaration {
        name,
        parameters,
        body: Box::new(Node::Scope(body)),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Node,
        tokens::{Token, TokenType},
    };

    use super::parse_function_declaration;

    #[test]
    fn test_parse_function_declaration() {
        assert_eq!(
            &parse_function_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword,
                        value: "fn".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier,
                        value: "main".to_string(),
                        start: (0, 3),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        value: "(".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        value: ")".to_string(),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        value: "{".to_string(),
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 11),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
                        start: (0, 15),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        value: "}".to_string(),
                        start: (0, 16),
                        end: (0, 16),
                    })
                ]
                .into_iter()
                .peekable(),
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
