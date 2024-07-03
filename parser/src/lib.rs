use core::panic;
use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};
use nodes::{Node, Operator, Program};

pub mod nodes;

const UNEXPECTED_ERROR: &str = "This does not happen, what the fuck are you doing?";
const UNEXPECTED_END_OF_INPUT_ERROR: &str = "Unexpected end of input!";

pub fn parse(tokens: &[Token]) -> Program {
    let mut program = Program {
        program: Vec::new(),
    };

    let mut tokens = tokens.iter().peekable();

    while let Some(_) = tokens.peek() {
        let node = convert(&mut program, &mut tokens);
        program.program.push(node);
    }

    program
}

fn convert<'a, I>(program: &mut Program, tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(
        tkn @ Token {
            token,
            value,
            start,
            end,
        },
    ) = tokens.next()
    {
        println!("{:?}", tkn);
        return match token {
            TokenType::Number => convert_number(tkn),
            TokenType::Operator => {
                let operator = match &value as &str {
                    "+" => Operator::Add,
                    "-" => Operator::Subtract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    "%" => Operator::Modulo,
                    _ => panic!("{}", UNEXPECTED_ERROR),
                };

                match program
                    .program
                    .pop()
                    .expect(&format!("[{}] Expected a number or an operator", start - 1))
                {
                    a @ Node::Number(_) => Node::Operation {
                        operator,
                        a: Box::new(a),
                        b: Box::new(convert_number(tokens.next().expect(UNEXPECTED_ERROR))),
                    },
                    a @ Node::Operation { .. } => {
                        extend_operation(a, operator, convert(program, tokens))
                    }
                }
            }
        };
    } else {
        panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
    }
}

fn convert_number(
    Token {
        token,
        value,
        start,
        end,
    }: &Token,
) -> Node {
    if let TokenType::Number = token {
        Node::Number(
            value
                .parse()
                .expect(&format!("[{start}-{end}] Invalid number: \"{value}\"")),
        )
    } else {
        panic!("{}", UNEXPECTED_ERROR);
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
        extend_operation,
        nodes::{Node, Operator},
        parse,
    };

    #[test]
    fn parse_numbers() {
        assert_eq!(
            parse(&[Token {
                token: TokenType::Number,
                value: "54".to_string(),
                start: 0,
                end: 2,
            }])
            .program,
            vec![Node::Number(54.)]
        );
        assert_eq!(
            parse(&[Token {
                token: TokenType::Number,
                value: "6.".to_string(),
                start: 0,
                end: 2,
            }])
            .program,
            vec![Node::Number(6.)]
        );
        assert_eq!(
            parse(&[Token {
                token: TokenType::Number,
                value: ".2".to_string(),
                start: 0,
                end: 2,
            }])
            .program,
            vec![Node::Number(0.2)]
        );
        assert_eq!(
            parse(&[Token {
                token: TokenType::Number,
                value: "8.5".to_string(),
                start: 0,
                end: 2,
            }])
            .program,
            vec![Node::Number(8.5)]
        );
    }

    #[test]
    fn parse_simple_operations() {
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
            ])
            .program,
            vec![Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(7.5)),
            }]
        );

        assert_eq!(
            parse(&[
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
            ])
            .program,
            vec![Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(0.3)),
                b: Box::new(Node::Number(4.)),
            }]
        );

        assert_eq!(
            parse(&[
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
            ])
            .program,
            vec![Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(2.)),
                b: Box::new(Node::Number(1.)),
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
            ])
            .program,
            vec![Node::Operation {
                operator: Operator::Modulo,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(1.5)),
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
    }

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
            ])
            .program,
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
            ])
            .program,
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
            ])
            .program,
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
            ])
            .program,
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
            ])
            .program,
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
                    value: "5.5".to_string(),
                    start: 4,
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
                    value: "8".to_string(),
                    start: 8,
                    end: 8,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 9,
                    end: 9,
                },
                Token {
                    token: TokenType::Number,
                    value: ".7".to_string(),
                    start: 10,
                    end: 12,
                }
            ])
            .program,
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
    }
}
