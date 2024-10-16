use std::{iter::Peekable, usize};

use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{literal_parser::parse_literal, operation_parser::parse_operation_greedy};

pub fn parse_identifier<'a, I>(
    tokens: &mut Peekable<I>,
    Token {
        token, value: name, ..
    }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    if let TokenType::Identifier = token {
        Ok(match tokens.peek() {
            Some(Token {
                token: TokenType::OpeningParenthesis,
                ..
            }) => {
                tokens.next();

                let mut arguments = Vec::new();

                loop {
                    match tokens.next() {
                        Some(Token {
                            token: TokenType::ClosingParenthesis,
                            ..
                        }) => {
                            return Ok(Node::FunctionCall {
                                name: name.to_owned(),
                                arguments,
                            });
                        }
                        Some(
                            literal @ Token {
                                token: TokenType::Identifier,
                                ..
                            },
                        ) => {
                            arguments.push(parse_identifier(tokens, literal)?);

                            match tokens.peek() {
                                Some(Token {
                                    token: TokenType::Operator,
                                    ..
                                }) => {
                                    let mut program = Vec::from([arguments.pop().unwrap()]);
                                    let next = tokens.next().unwrap();
                                    arguments.push(parse_operation_greedy(
                                        &mut program,
                                        tokens,
                                        next,
                                    )?);
                                }
                                Some(Token {
                                    token: TokenType::Comma,
                                    ..
                                }) => {
                                    tokens.next();
                                }
                                Some(Token {
                                    token: TokenType::ClosingParenthesis,
                                    ..
                                }) => {
                                    tokens.next();
                                    return Ok(Node::FunctionCall {
                                        name: name.to_owned(),
                                        arguments,
                                    });
                                }
                                Some(Token { token, start, .. }) => {
                                    Err(ParserErrors::UnexpectedToken {
                                        token: *token,
                                        loc: *start,
                                    })?
                                }
                                None => Err(ParserErrors::EndOfInput {
                                    loc: (usize::MAX, usize::MAX),
                                })?,
                            }
                        }
                        Some(
                            token @ Token {
                                token: TokenType::Literal,
                                ..
                            },
                        ) => {
                            arguments.push(parse_literal(token)?);

                            match tokens.peek() {
                                Some(Token {
                                    token: TokenType::Operator,
                                    ..
                                }) => {
                                    let mut program = Vec::from([arguments.pop().unwrap()]);
                                    let next = tokens.next().unwrap();
                                    arguments.push(parse_operation_greedy(
                                        &mut program,
                                        tokens,
                                        next,
                                    )?);
                                }
                                Some(Token {
                                    token: TokenType::Comma,
                                    ..
                                }) => {
                                    tokens.next();
                                }
                                Some(Token {
                                    token: TokenType::ClosingParenthesis,
                                    ..
                                }) => {
                                    tokens.next().unwrap();
                                    return Ok(Node::FunctionCall {
                                        name: name.to_owned(),
                                        arguments,
                                    });
                                }
                                Some(Token { token, start, .. }) => {
                                    Err(ParserErrors::UnexpectedToken {
                                        token: *token,
                                        loc: *start,
                                    })?
                                }
                                None => Err(ParserErrors::EndOfInput {
                                    loc: (usize::MAX, usize::MAX),
                                })?,
                            }
                        }
                        Some(Token { token, start, .. }) => Err(ParserErrors::UnexpectedToken {
                            token: *token,
                            loc: *start,
                        })?,
                        None => Err(ParserErrors::EndOfInput {
                            loc: (usize::MAX, usize::MAX),
                        })?,
                    }
                }
            }
            _ => Node::VariableReference(name.to_owned()),
        })
    } else {
        Err(ParserErrors::ThisNeverHappens)?
    }
}
