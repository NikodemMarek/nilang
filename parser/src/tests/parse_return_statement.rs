use crate::{
    nodes::{Node, Operator},
    parse,
};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_return_statement() {
    assert_eq!(
        parse(&[
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
        ]),
        vec![Node::Return(Box::new(Node::Number(6.)))]
    );
    assert_eq!(
        parse(&[
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
        ]),
        vec![Node::Return(Box::new(Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Number(9.)),
        }))]
    );
    assert_eq!(
        parse(&[
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
        ]),
        vec![Node::Return(Box::new(Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Number(9.)),
        }))]
    );
    assert_eq!(
        parse(&[
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
        ]),
        vec![Node::Return(Box::new(Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }),
            b: Box::new(Node::Number(5.)),
        }))]
    );
    assert_eq!(
        parse(&[
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
                value: "*".to_string(),
                start: 6,
                end: 6,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 7,
                end: 7,
            },
        ]),
        vec![Node::Return(Box::new(Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(9.)),
                b: Box::new(Node::Number(5.)),
            }),
        }))]
    );
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::Keyword,
                value: "rt".to_string(),
                start: 0,
                end: 1,
            },
            Token {
                token: TokenType::OpeningBrace,
                value: "{".to_string(),
                start: 3,
                end: 3,
            },
            Token {
                token: TokenType::Keyword,
                value: "rt".to_string(),
                start: 4,
                end: 5,
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: 7,
                end: 7,
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 8,
                end: 8,
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: 9,
                end: 9,
            },
            Token {
                token: TokenType::ClosingBrace,
                value: "}".to_string(),
                start: 10,
                end: 10,
            }
        ]),
        vec![Node::Return(Box::new(Node::Scope(vec![Node::Return(
            Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            })
        )])))]
    );
}
