use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::{
    nodes::Node, parsers::operation_parser::parse_operation_greedy, UNEXPECTED_END_OF_INPUT_ERROR,
};

use super::parse;

pub fn parse_return<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token { .. }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    Node::Return(Box::new({
        let tree = parse(program, tokens);

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
                let operation = parse_operation_greedy(&mut Vec::from([tree]), tokens, token);

                match tokens.peek() {
                    Some(Token {
                        token: TokenType::Semicolon,
                        ..
                    }) => {
                        tokens.next();
                        operation
                    }
                    Some(Token { end, .. }) => {
                        panic!("[{}] Expected a semicolon", end + 1);
                    }
                    None => {
                        panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
                    }
                }
            }
            Some(Token { end, .. }) => {
                panic!("[{}] Expected a semicolon", end + 1);
            }
            None => {
                panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{
        nodes::{Node, Operator},
        parse,
    };

    #[test]
    fn parse_return_statement() {
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: 0,
                    end: 1,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::Semicolon,
                    value: ";".to_string(),
                    start: 4,
                    end: 4,
                },
            ]),
            &[Node::Return(Box::new(Node::Number(6.)))]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: 0,
                    end: 1,
                },
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 4,
                    end: 4,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 6,
                    end: 6,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 7,
                    end: 7,
                },
                Token {
                    token: TokenType::Semicolon,
                    value: ";".to_string(),
                    start: 8,
                    end: 8,
                },
            ]),
            &[Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: 0,
                    end: 1,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 4,
                    end: 4,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::Semicolon,
                    value: ";".to_string(),
                    start: 6,
                    end: 6,
                },
            ]),
            &[Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Keyword,
                    value: "rt".to_string(),
                    start: 0,
                    end: 1,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 4,
                    end: 4,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 6,
                    end: 6,
                },
                Token {
                    token: TokenType::Number,
                    value: "5".to_string(),
                    start: 7,
                    end: 7,
                },
                Token {
                    token: TokenType::Semicolon,
                    value: ";".to_string(),
                    start: 8,
                    end: 8,
                },
            ]),
            &[Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }))]
        );
    }
}
