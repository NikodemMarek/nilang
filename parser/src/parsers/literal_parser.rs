use std::{iter::Peekable, usize};

use errors::ParserErrors;
use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

use super::{number_parser::parse_number, operation_parser::parse_operation_greedy};

pub fn parse_literal<'a, I>(
    tokens: &mut Peekable<I>,
    Token { token, value, .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    if let TokenType::Literal = token {
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
                                name: value.to_owned(),
                                arguments,
                            });
                        }
                        Some(
                            literal @ Token {
                                token: TokenType::Literal,
                                ..
                            },
                        ) => {
                            arguments.push(parse_literal(tokens, literal)?);

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
                                }) => (),
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
                                token: TokenType::Number,
                                ..
                            },
                        ) => {
                            arguments.push(parse_number(token)?);

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
                                }) => (),
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
            _ => Node::VariableReference(value.to_owned()),
        })
    } else {
        Err(ParserErrors::ThisNeverHappens)?
    }
}
