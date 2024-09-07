use std::iter::Peekable;

use nilang_lexer::tokens::Token;

use crate::{
    nodes::{Node, Operator},
    UNEXPECTED_ERROR,
};

use super::parse;

pub fn parse_operation<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token {
        token: _,
        value,
        start,
        end: _,
    }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    let operator = match value.as_str() {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "%" => Operator::Modulo,
        _ => panic!("{}", UNEXPECTED_ERROR),
    };

    match program
        .pop()
        .unwrap_or_else(|| panic!("[{}] Expected a number or a variable", start - 1))
    {
        a @ Node::Number(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)),
        },
        a @ Node::VariableReference(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)),
        },
        a @ Node::Operation { .. } => extend_operation(a, operator, parse(program, tokens)),
        Node::Return(value) => Node::Return(Box::new(match *value {
            a @ Node::Number(_) | a @ Node::VariableReference(_) => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(parse(program, tokens)),
            },
            a @ Node::Operation { .. } => extend_operation(a, operator, parse(program, tokens)),
            a @ Node::Scope(_) => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(parse(program, tokens)),
            },
            Node::Return(_)
            | Node::FunctionDeclaration { .. }
            | Node::VariableDeclaration { .. } => {
                panic!("{}", UNEXPECTED_ERROR)
            }
        })),
        a @ Node::Scope(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)),
        },
        Node::FunctionDeclaration { .. } | Node::VariableDeclaration { .. } => {
            panic!("[{}] Unexpected token", start - 1)
        }
    }
}

fn extend_operation(operation: Node, operator: Operator, node: Node) -> Node {
    if let Node::Operation {
        operator: prev_operator,
        a: prev_a,
        b: prev_b,
    } = operation
    {
        match operator {
            Operator::Add | Operator::Subtract => match prev_operator {
                Operator::Add | Operator::Subtract => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
            },
            Operator::Multiply | Operator::Divide | Operator::Modulo => match prev_operator {
                Operator::Add | Operator::Subtract => Node::Operation {
                    operator: prev_operator,
                    a: prev_a,
                    b: Box::new(Node::Operation {
                        operator,
                        a: prev_b,
                        b: Box::new(node),
                    }),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
            },
        }
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{
        nodes::{Node, Operator},
        parse,
        parsers::operation_parser::extend_operation,
    };

    #[test]
    fn parse_simple_operations() {
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
                    token: TokenType::Number,
                    value: "9".to_string(),
                    start: 2,
                    end: 2,
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
            &[Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(7.5)),
            }]
        );

        assert_eq!(
            &parse(&[
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
            &[Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(0.3)),
                b: Box::new(Node::Number(4.)),
            }]
        );

        assert_eq!(
            &parse(&[
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
            &[Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(2.)),
                b: Box::new(Node::Number(1.)),
            }]
        );

        assert_eq!(
            &parse(&[
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
            &[Node::Operation {
                operator: Operator::Modulo,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(1.5)),
            }]
        );
    }

    #[test]
    fn parse_complex_operations() {
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
            &[Node::Operation {
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
            &[Node::Operation {
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
            &[Node::Operation {
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
            &[Node::Operation {
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
            &parse(&[
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
            &[Node::Operation {
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
            &parse(&[
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
            &[Node::Operation {
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
            &parse(&[
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
            &[Node::Operation {
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
            &parse(&[
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
            &[Node::Operation {
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

    #[test]
    fn extend_complex_operation() {
        assert_eq!(
            extend_operation(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                },
                Operator::Add,
                Node::Number(4.)
            ),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                }),
                b: Box::new(Node::Number(4.))
            }
        );
        assert_eq!(
            extend_operation(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                },
                Operator::Multiply,
                Node::Number(4.)
            ),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(8.)),
                    b: Box::new(Node::Number(4.))
                })
            }
        );
        assert_eq!(
            extend_operation(
                Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                },
                Operator::Add,
                Node::Number(4.)
            ),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                }),
                b: Box::new(Node::Number(4.))
            }
        );
        assert_eq!(
            extend_operation(
                Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                },
                Operator::Multiply,
                Node::Number(4.)
            ),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                }),
                b: Box::new(Node::Number(4.))
            }
        );
        assert_eq!(
            extend_operation(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                },
                Operator::Subtract,
                Node::Scope(vec![Node::Return(Box::new(Node::Number(4.)))]),
            ),
            Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(8.))
                }),
                b: Box::new(Node::Scope(vec![Node::Return(Box::new(Node::Number(4.)))]))
            }
        );
    }
}
