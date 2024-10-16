use std::iter::Peekable;

use errors::ParserErrors;
use nilang_types::{
    nodes::{Node, Operator},
    tokens::{Token, TokenType},
};

use super::parse;

pub fn parse_operation_greedy<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    token: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    let operation = parse_operation(program, tokens, token)?;

    if let Some(Token {
        token: TokenType::Operator,
        ..
    }) = tokens.peek()
    {
        let operator = tokens.next().unwrap();
        parse_operation_greedy(&mut Vec::from([operation]), tokens, operator)
    } else {
        Ok(operation)
    }
}

pub fn parse_operation<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token { value, start, .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    let operator = match value.as_str() {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "%" => Operator::Modulo,
        _ => Err(ParserErrors::ThisNeverHappens)?,
    };

    let preceeding = match program.pop() {
        Some(node) => node,
        None => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Number, TokenType::Literal]),
            loc: (start.0, start.1 - 1),
        })?,
    };
    Ok(match preceeding {
        a @ Node::Number(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)?),
        },
        a @ Node::VariableReference(_) | a @ Node::FunctionCall { .. } => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)?),
        },
        a @ Node::Operation { .. } => extend_operation(a, operator, parse(program, tokens)?)?,
        Node::Return(value) => Node::Return(Box::new(match *value {
            a @ Node::Number(_)
            | a @ Node::VariableReference(_)
            | a @ Node::FunctionCall { .. } => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(parse(program, tokens)?),
            },
            a @ Node::Operation { .. } => extend_operation(a, operator, parse(program, tokens)?)?,
            a @ Node::Scope(_) => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(parse(program, tokens)?),
            },
            Node::Return(_)
            | Node::FunctionDeclaration { .. }
            | Node::VariableDeclaration { .. } => Err(ParserErrors::ThisNeverHappens)?,
        })),
        a @ Node::Scope(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(program, tokens)?),
        },
        Node::FunctionDeclaration { .. } | Node::VariableDeclaration { .. } => {
            Err(ParserErrors::InvalidOperand {
                loc: (start.0, start.1 - 1),
            })?
        }
    })
}

fn extend_operation(operation: Node, operator: Operator, node: Node) -> eyre::Result<Node> {
    if let Node::Operation {
        operator: prev_operator,
        a: prev_a,
        b: prev_b,
    } = operation
    {
        Ok(match operator {
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
        })
    } else {
        Err(ParserErrors::ThisNeverHappens)?
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::operation_parser::{
        extend_operation, parse_operation, parse_operation_greedy,
    };

    #[test]
    fn parse_operations_greedy() {
        let tokens = [
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }
        );
    }

    #[test]
    fn parse_simple_operations() {
        let tokens = [Token {
            token: TokenType::Number,
            value: "9".to_string(),
            start: (0, 2),
            end: (0, 2),
        }];
        assert_eq!(
            parse_operation(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }
        );

        let tokens = [Token {
            token: TokenType::Number,
            value: "7.5".to_string(),
            start: (0, 2),
            end: (0, 4),
        }];
        assert_eq!(
            parse_operation(
                &mut Vec::from([Node::Number(5.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "-".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(7.5)),
            }
        );

        let tokens = [Token {
            token: TokenType::Number,
            value: "4".to_string(),
            start: (0, 4),
            end: (0, 4),
        }];
        assert_eq!(
            parse_operation(
                &mut Vec::from([Node::Number(0.3)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: (0, 3),
                    end: (0, 3),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(0.3)),
                b: Box::new(Node::Number(4.)),
            }
        );

        let tokens = [Token {
            token: TokenType::Number,
            value: "1".to_string(),
            start: (0, 2),
            end: (0, 2),
        }];
        assert_eq!(
            parse_operation(
                &mut Vec::from([Node::Number(2.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "/".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(2.)),
                b: Box::new(Node::Number(1.)),
            }
        );

        let tokens = [Token {
            token: TokenType::Number,
            value: "1.5".to_string(),
            start: (0, 2),
            end: (0, 4),
        }];
        assert_eq!(
            parse_operation(
                &mut Vec::from([Node::Number(5.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "%".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Modulo,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(1.5)),
            }
        );
    }

    #[test]
    fn parse_complex_operations() {
        let tokens = [
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Operator,
                value: "-".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: (0, 2),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "7".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                }
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(0.5)),
                }),
                b: Box::new(Node::Number(7.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: (0, 2),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "/".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "7".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(0.5)),
                }),
                b: Box::new(Node::Number(7.0)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: (0, 2),
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
                value: "7".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(0.5)),
                }),
                b: Box::new(Node::Number(7.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: ".5".to_string(),
                start: (0, 2),
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
                value: "7".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::Number,
                value: "3".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(6.)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "/".to_string(),
                    start: (0, 1),
                    end: (0, 1),
                },
            )
            .unwrap(),
            Node::Operation {
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
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "5.5".to_string(),
                start: (0, 3),
                end: (0, 5),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::Number,
                value: "8".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 8),
                end: (0, 8),
            },
            Token {
                token: TokenType::Number,
                value: ".7".to_string(),
                start: (0, 9),
                end: (0, 11),
            },
        ];
        assert_eq!(
            parse_operation_greedy(
                &mut Vec::from([Node::Number(0.2)]),
                &mut tokens.iter().peekable(),
                &Token {
                    token: TokenType::Operator,
                    value: "-".to_string(),
                    start: (0, 2),
                    end: (0, 2),
                }
            )
            .unwrap(),
            Node::Operation {
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
            }
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
            )
            .unwrap(),
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
            )
            .unwrap(),
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
            )
            .unwrap(),
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
            )
            .unwrap(),
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
            )
            .unwrap(),
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
