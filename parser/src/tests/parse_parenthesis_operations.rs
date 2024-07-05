use crate::{
    nodes::{Node, Operator},
    parse,
};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_parenthesis_operations() {
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
                value: ")".to_string(),
                start: 4,
                end: 4,
            },
        ])
        .program,
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Number(9.)),
        }]
    );
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
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
        ])
        .program,
        vec![Node::Operation {
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
        parse(&[
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
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
                value: ")".to_string(),
                start: 6,
                end: 6,
            },
        ])
        .program,
        vec![Node::Operation {
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
        parse(&[
            Token {
                token: TokenType::LeftParenthesis,
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
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
                value: ")".to_string(),
                start: 7,
                end: 7,
            },
            Token {
                token: TokenType::RightParenthesis,
                value: ")".to_string(),
                start: 8,
                end: 8,
            },
        ])
        .program,
        vec![Node::Operation {
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
        parse(&[
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
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
                value: ")".to_string(),
                start: 6,
                end: 6,
            },
        ])
        .program,
        vec![Node::Operation {
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
        parse(&[
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
                token: TokenType::LeftParenthesis,
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
                token: TokenType::RightParenthesis,
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
        ])
        .program,
        vec![Node::Operation {
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
