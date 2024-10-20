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

pub fn parse_function_arguments<I>(tokens: &mut Peekable<I>) -> Result<Vec<Node>, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
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

    Ok({
        let mut arguments = Vec::new();

        loop {
            match tokens.peek() {
                Some(Ok(Token {
                    token: TokenType::Literal(_),
                    ..
                })) => {
                    let literal = parse_literal(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, literal?)?);

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
                    token: TokenType::Identifier(_),
                    ..
                })) => {
                    let identifier = parse_identifier(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, identifier?)?);

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
                    token: TokenType::OpeningParenthesis,
                    ..
                })) => {
                    let parenthesis = parse_parenthesis(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, parenthesis?)?);

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
                })) => {
                    tokens.next();
                    break;
                }
                Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([
                        TokenType::Identifier("".into()),
                        TokenType::Literal("".into()),
                        TokenType::OpeningParenthesis,
                        TokenType::ClosingParenthesis,
                    ]),
                    loc: *start,
                })?,
                Some(_) | None => Err(ParserErrors::EndOfInput {
                    loc: (usize::MAX, usize::MAX),
                })?,
            }
        }

        arguments
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::function_arguments_parser::parse_function_arguments;

    #[test]
    fn test_parse_function_arguments() {
        assert_eq!(
            parse_function_arguments(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            vec![Node::Number(5.), Node::VariableReference("x".to_string()),]
        );

        assert_eq!(
            parse_function_arguments(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Vec::from([Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::VariableReference("x".to_string())),
                b: Box::new(Node::Number(4.)),
            }])
        );
    }
}
