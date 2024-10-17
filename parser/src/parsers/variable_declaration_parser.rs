use std::iter::Peekable;

use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{operation_parser::parse_operation_greedy, parse};

pub fn parse_variable_declaration<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token { end, .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    Ok(Node::VariableDeclaration {
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
        value: Box::new({
            if let Some(Token {
                token: TokenType::Equals,
                ..
            }) = tokens.next()
            {
                match parse(program, tokens)? {
                    node @ Node::Number(_)
                    | node @ Node::VariableReference(_)
                    | node @ Node::FunctionCall { .. } => match tokens.peek() {
                        Some(Token {
                            token: TokenType::Semicolon,
                            ..
                        }) => {
                            tokens.next();
                            node
                        }
                        Some(Token {
                            token: TokenType::Operator,
                            ..
                        }) => {
                            program.push(node);
                            let token = tokens.next().unwrap();
                            let node = parse_operation_greedy(program, tokens, token)?;

                            if let Some(Token {
                                token: TokenType::Semicolon,
                                ..
                            }) = tokens.peek()
                            {
                                tokens.next();
                            } else {
                                Err(ParserErrors::ExpectedTokens {
                                    tokens: Vec::from([TokenType::Semicolon]),
                                    loc: (end.0, end.1 + 1),
                                })?
                            }

                            node
                        }
                        _ => Err(ParserErrors::ExpectedTokens {
                            tokens: Vec::from([TokenType::Semicolon, TokenType::Operator]),
                            loc: (end.0, end.1 + 1),
                        })?,
                    },
                    node @ Node::Operation { .. } => {
                        if let Some(Token {
                            token: TokenType::Semicolon,
                            ..
                        }) = tokens.peek()
                        {
                            tokens.next();
                        } else {
                            Err(ParserErrors::ExpectedTokens {
                                tokens: Vec::from([TokenType::Semicolon]),
                                loc: (end.0, end.1 + 1),
                            })?
                        }

                        node
                    }
                    _ => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([
                            TokenType::Literal,
                            TokenType::Keyword,
                            TokenType::Operator,
                        ]),
                        loc: (end.0, end.1 + 1),
                    })?,
                }
            } else {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::Equals]),
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

    use crate::parsers::variable_declaration_parser::parse_variable_declaration;

    #[test]
    fn parse_variable_declaration_statement() {
        let tokens = [
            Token {
                token: TokenType::Identifier,
                value: "test".to_string(),
                start: (0, 1),
                end: (0, 4),
            },
            Token {
                token: TokenType::Equals,
                value: "=".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Literal,
                value: "9".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::Semicolon,
                value: ";".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
        ];

        assert_eq!(
            parse_variable_declaration(
                &mut Vec::new(),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Identifier,
                    value: "vr".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Number(9.))
            }
        );
    }
}
