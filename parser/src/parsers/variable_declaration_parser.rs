use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

use super::{operation_parser::parse_operation, parse};

pub fn parse_variable_declaration<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token {
        token: _,
        value: _,
        start: _,
        end,
    }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    Node::VariableDeclaration {
        name: match tokens.next() {
            Some(Token {
                token: TokenType::Literal,
                value,
                ..
            }) => value.to_owned(),
            _ => panic!("[{}] Expected a variable name", end + 1),
        },
        value: Box::new({
            if let Some(Token {
                token: TokenType::Equals,
                ..
            }) = tokens.next()
            {
                match parse(program, tokens) {
                    node @ Node::Number(_) | node @ Node::VariableReference(_) => {
                        match tokens.peek() {
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
                                let node = parse_operation(program, tokens, token);

                                if let Some(Token {
                                    token: TokenType::Semicolon,
                                    ..
                                }) = tokens.peek()
                                {
                                    tokens.next();
                                } else {
                                    panic!("[{}] Expected a semicolon", end + 1);
                                }

                                node
                            }
                            _ => {
                                panic!("[{}] Expected a semicolon, or an operator", end + 1);
                            }
                        }
                    }
                    node @ Node::Operation { .. } => {
                        if let Some(Token {
                            token: TokenType::Semicolon,
                            ..
                        }) = tokens.peek()
                        {
                            tokens.next();
                        } else {
                            panic!("[{}] Expected a semicolon", end + 1);
                        }

                        node
                    }
                    _ => panic!(
                        "[{}] Expected a number, variable reference or operation",
                        end + 1
                    ),
                }
            } else {
                panic!("[{}] Expected an equals sign", end + 1)
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{nodes::Node, parse};

    #[test]
    fn parse_variable_declaration() {
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Keyword,
                    value: "vr".to_string(),
                    start: 0,
                    end: 1,
                },
                Token {
                    token: TokenType::Literal,
                    value: "test".to_string(),
                    start: 1,
                    end: 4,
                },
                Token {
                    token: TokenType::Equals,
                    value: "=".to_string(),
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
                    token: TokenType::Semicolon,
                    value: ";".to_string(),
                    start: 7,
                    end: 7,
                },
            ]),
            &[Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Number(9.))
            }]
        );
    }
}
