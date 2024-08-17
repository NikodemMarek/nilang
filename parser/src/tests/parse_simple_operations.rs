use crate::{
    nodes::{Node, Operator},
    parse_tokens,
};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_simple_operations() {
    assert_eq!(
        parse_tokens(&[
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
                token: TokenType::Number,
                value: "9".to_string(),
                start: 2,
                end: 2,
            },
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Number(9.)),
        }]
    );

    assert_eq!(
        parse_tokens(&[
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Operator,
                value: "-".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: "7.5".to_string(),
                start: 2,
                end: 4,
            },
        ]),
        vec![Node::Operation {
            operator: Operator::Subtract,
            a: Box::new(Node::Number(5.)),
            b: Box::new(Node::Number(7.5)),
        }]
    );

    assert_eq!(
        parse_tokens(&[
            Token {
                token: TokenType::Number,
                value: "0.3".to_string(),
                start: 0,
                end: 2,
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: 3,
                end: 3,
            },
            Token {
                token: TokenType::Number,
                value: "4".to_string(),
                start: 4,
                end: 4,
            },
        ]),
        vec![Node::Operation {
            operator: Operator::Multiply,
            a: Box::new(Node::Number(0.3)),
            b: Box::new(Node::Number(4.)),
        }]
    );

    assert_eq!(
        parse_tokens(&[
            Token {
                token: TokenType::Number,
                value: "2".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Operator,
                value: "/".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: "1".to_string(),
                start: 2,
                end: 2,
            },
        ]),
        vec![Node::Operation {
            operator: Operator::Divide,
            a: Box::new(Node::Number(2.)),
            b: Box::new(Node::Number(1.)),
        }]
    );

    assert_eq!(
        parse_tokens(&[
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Operator,
                value: "%".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: "1.5".to_string(),
                start: 2,
                end: 4,
            },
        ]),
        vec![Node::Operation {
            operator: Operator::Modulo,
            a: Box::new(Node::Number(5.)),
            b: Box::new(Node::Number(1.5)),
        }]
    );
}
