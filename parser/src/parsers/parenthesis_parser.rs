use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, UNEXPECTED_ERROR};

use super::parse;

pub fn parse_parenthesis<'a, I>(tokens: &mut Peekable<I>, (start, end): (&usize, &usize)) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    let mut in_parenthesis = Vec::new();

    let mut last_node_end = end;
    while let Some(token) = tokens.peek() {
        last_node_end = end;
        if token.token == TokenType::ClosingParenthesis {
            tokens.next();
            break;
        } else {
            let node = parse(&mut in_parenthesis, tokens);
            in_parenthesis.push(node);
        }
    }

    if in_parenthesis.is_empty() {
        panic!("[{}-{}] Empty parenthesis", start, last_node_end)
    }
    if in_parenthesis.len() > 1 {
        panic!(
            "[{}-{}] Invalid operation in parenthesis",
            start, last_node_end
        )
    }

    in_parenthesis.first().expect(UNEXPECTED_ERROR).to_owned()
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{
        nodes::{Node, Operator},
        parse,
    };

    #[test]
    fn parse_parenthesis_operations() {
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 4,
                    end: 4,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
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
                    value: "5".to_string(),
                    start: 6,
                    end: 6,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
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
                    value: "5".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 6,
                    end: 6,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 3,
                    end: 3,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
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
                    value: "5".to_string(),
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
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 8,
                    end: 8,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
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
                    value: "5".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 6,
                    end: 6,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }]
        );
        assert_eq!(
            &parse(&[
                Token {
                    token: TokenType::Number,
                    value: "6".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::OpeningParenthesis,
                    value: "(".to_string(),
                    start: 2,
                    end: 2,
                },
                Token {
                    token: TokenType::Number,
                    value: "9".to_string(),
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
                    value: "5".to_string(),
                    start: 5,
                    end: 5,
                },
                Token {
                    token: TokenType::ClosingParenthesis,
                    value: ")".to_string(),
                    start: 6,
                    end: 6,
                },
                Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: 7,
                    end: 7,
                },
                Token {
                    token: TokenType::Number,
                    value: "7".to_string(),
                    start: 8,
                    end: 8,
                },
            ]),
            &[Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(9.)),
                        b: Box::new(Node::Number(5.)),
                    }),
                }),
                b: Box::new(Node::Number(7.)),
            }]
        );
    }
}
