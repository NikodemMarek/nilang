use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows, parenthesis_parser::parse_parenthesis,
};

pub fn parse_variable_declaration<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Keyword(value),
            ..
        })) => {
            if *value != *"vr" {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::Keyword("vr".into())]),
                    loc: (0, 1),
                })?
            }
        }
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Equals]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let name = match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Identifier(value),
            ..
        })) => value,
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Identifier("".into())]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Equals,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Equals]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let value = match tokens.peek() {
        Some(Ok(Token {
            token: TokenType::Literal(_),
            ..
        })) => {
            let literal = parse_literal(tokens);
            parse_operation_if_operator_follows(tokens, literal?)?
        }
        Some(Ok(Token {
            token: TokenType::Identifier(_),
            ..
        })) => {
            let identifier = parse_identifier(tokens);
            parse_operation_if_operator_follows(tokens, identifier?)?
        }
        Some(Ok(Token {
            token: TokenType::OpeningParenthesis,
            ..
        })) => {
            let parenthesis = parse_parenthesis(tokens);
            parse_operation_if_operator_follows(tokens, parenthesis?)?
        }
        Some(Ok(Token { end, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([
                TokenType::Literal("".into()),
                TokenType::Identifier("".into()),
                TokenType::OpeningParenthesis,
            ]),
            loc: *end,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Semicolon,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Semicolon]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    Ok(Node::VariableDeclaration {
        name: name.to_string(),
        value: Box::new(value),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::variable_declaration_parser::parse_variable_declaration;

    #[test]
    fn test_parse_variable_declaration() {
        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword("vr".into()),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Number(9.))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword("vr".into()),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 5),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 7),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 12),
                        end: (0, 12),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::VariableReference("test2".into()))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword("vr".into()),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 5),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 11),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 12),
                        end: (0, 12),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                })
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword("vr".into()),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 5),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 8),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 13),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 16),
                        end: (0, 16),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::VariableReference("test2".to_string())),
                    b: Box::new(Node::Number(9.)),
                })
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword("vr".into()),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 5),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("abc".into()),
                        start: (0, 7),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 11),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 12),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 13),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 14),
                        end: (0, 14),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::FunctionCall {
                    name: "abc".to_string(),
                    arguments: vec![Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(6.)),
                        b: Box::new(Node::Number(9.)),
                    }]
                })
            }
        );
    }
}
