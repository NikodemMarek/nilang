use std::iter::Peekable;

use errors::ParserErrors;
use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, parsers::operation_parser::parse_operation_greedy};

use super::parse;

pub fn parse_return<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token { .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    Ok(Node::Return(Box::new({
        let tree = parse(program, tokens)?;

        match tokens.peek() {
            Some(Token {
                token: TokenType::Semicolon,
                ..
            }) => {
                tokens.next();
                tree
            }
            Some(Token {
                token: TokenType::Operator,
                ..
            }) => {
                let token = tokens.next().unwrap();
                let operation = parse_operation_greedy(&mut Vec::from([tree]), tokens, token)?;

                match tokens.peek() {
                    Some(Token {
                        token: TokenType::Semicolon,
                        ..
                    }) => {
                        tokens.next();
                        operation
                    }
                    Some(Token { end, .. }) => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Semicolon]),
                        loc: (end.0, end.1 + 1),
                    })?,
                    None => Err(ParserErrors::EndOfInput {
                        loc: (usize::MAX, usize::MAX),
                    })?,
                }
            }
            Some(Token { end, .. }) => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Semicolon]),
                loc: (end.0, end.1 + 1),
            })?,
            None => Err(ParserErrors::EndOfInput {
                loc: (usize::MAX, usize::MAX),
            })?,
        }
    })))
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{
        nodes::{Node, Operator},
        parsers::return_parser::parse_return,
    };

    #[test]
    fn parse_return_statement() {
        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Semicolon,
                value: ";".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ];
        assert_eq!(
            parse_return(
                &mut Vec::new(),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Return(Box::new(Node::Number(6.)))
        );

        let tokens = [
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
            Token {
                token: TokenType::Semicolon,
                value: ";".to_string(),
                start: (0, 8),
                end: (0, 8),
            },
        ];
        assert_eq!(
            parse_return(
                &mut Vec::new(),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Semicolon,
                value: ";".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ];
        assert_eq!(
            parse_return(
                &mut Vec::new(),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
            Token {
                token: TokenType::Semicolon,
                value: ";".to_string(),
                start: (0, 8),
                end: (0, 8),
            },
        ];
        assert_eq!(
            parse_return(
                &mut Vec::new(),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: (0, 0),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }))
        );
    }
}
