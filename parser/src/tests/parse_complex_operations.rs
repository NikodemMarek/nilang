use crate::{
    nodes::{Node, Operator},
    parse,
};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_complex_operations() {
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
                token: TokenType::Number,
                value: "9".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 3,
                end: 3,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 4,
                end: 4,
            }
        ]),
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
                token: TokenType::Number,
                value: "9".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::Operator,
                value: "-".to_string(),
                start: 3,
                end: 3,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 4,
                end: 4,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Subtract,
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
                value: "*".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: 2,
                end: 3,
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: 4,
                end: 4,
            },
            Token {
                token: TokenType::Number,
                value: "7".to_string(),
                start: 5,
                end: 5,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Multiply,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(0.5)),
            }),
            b: Box::new(Node::Number(7.)),
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
                token: TokenType::Number,
                value: ".5".to_string(),
                start: 2,
                end: 3,
            },
            Token {
                token: TokenType::Operator,
                value: "/".to_string(),
                start: 4,
                end: 4,
            },
            Token {
                token: TokenType::Number,
                value: "7".to_string(),
                start: 5,
                end: 5,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Divide,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(0.5)),
            }),
            b: Box::new(Node::Number(7.0)),
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
                token: TokenType::Number,
                value: ".5".to_string(),
                start: 2,
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
                value: "7".to_string(),
                start: 5,
                end: 5,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(0.5)),
            }),
            b: Box::new(Node::Number(7.)),
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
                value: "/".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: 2,
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
                value: "7".to_string(),
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
                value: "3".to_string(),
                start: 7,
                end: 7,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(0.5)),
            }),
            b: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(7.)),
                b: Box::new(Node::Number(3.)),
            }),
        }]
    );
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::Number,
                value: ".2".to_string(),
                start: 0,
                end: 1,
            },
            Token {
                token: TokenType::Operator,
                value: "-".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::Number,
                value: "5.5".to_string(),
                start: 3,
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
                value: "8".to_string(),
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
                value: ".7".to_string(),
                start: 9,
                end: 11,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(0.2)),
                b: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(5.5)),
                    b: Box::new(Node::Number(8.)),
                }),
            }),
            b: Box::new(Node::Number(0.7)),
        }]
    );
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::OpeningBrace,
                value: "{".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Keyword,
                value: "rt".to_string(),
                start: 1,
                end: 2,
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
                token: TokenType::ClosingBrace,
                value: "}".to_string(),
                start: 6,
                end: 6,
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 7,
                end: 7,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 8,
                end: 8,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Scope(vec![Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))])),
            b: Box::new(Node::Number(5.)),
        }]
    );
    assert_eq!(
        parse(&[
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
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
                token: TokenType::OpeningBrace,
                value: "{".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::Keyword,
                value: "rt".to_string(),
                start: 3,
                end: 4,
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
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
                value: "9".to_string(),
                start: 7,
                end: 7,
            },
            Token {
                token: TokenType::ClosingBrace,
                value: "}".to_string(),
                start: 8,
                end: 8,
            }
        ]),
        vec![Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(5.)),
            b: Box::new(Node::Scope(vec![Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))])),
        }]
    );
}