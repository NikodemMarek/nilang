use errors::ParserErrors;
use nilang_types::{
    nodes::{Node, Operator},
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse;

pub fn parse_operation_if_operator_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    node: Node,
) -> Result<Node, ParserErrors> {
    Ok(
        if let Token {
            token: TokenType::Operator(_),
            ..
        } = tokens.peek_valid()?
        {
            let operation = parse_operation(tokens, node, true);
            parse_operation_if_operator_follows(tokens, operation?)?
        } else {
            node
        },
    )
}

pub fn parse_operation_if_operator_follows_no_rearrange<I: PeekableAssumingIterator>(
    tokens: &mut I,
    node: Node,
) -> Result<Node, ParserErrors> {
    Ok(
        if let Token {
            token: TokenType::Operator(_),
            ..
        } = tokens.peek_valid()?
        {
            let operation = parse_operation(tokens, node, false);
            parse_operation_if_operator_follows(tokens, operation?)?
        } else {
            node
        },
    )
}

fn parse_operation<I: PeekableAssumingIterator>(
    tokens: &mut I,
    preceeding: Node,
    rearrange: bool,
) -> Result<Node, ParserErrors> {
    let (start, _, operator) = tokens.assume_operator()?;

    Ok(match preceeding {
        a @ Node::Number(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(tokens)?),
        },
        Node::FieldAccess { .. } => todo!(),
        a @ Node::VariableReference(_) | a @ Node::FunctionCall { .. } => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(tokens)?),
        },
        a @ Node::Operation { .. } => {
            let following = parse(tokens)?;
            if rearrange {
                extend_operation(a, operator, following)?
            } else {
                Node::Operation {
                    operator,
                    a: Box::new(a),
                    b: Box::new(following),
                }
            }
        }
        a @ Node::Scope(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse(tokens)?),
        },
        Node::Return(_)
        | Node::FunctionDeclaration { .. }
        | Node::VariableDeclaration { .. }
        | Node::Structure { .. }
        | Node::Object { .. } => Err(ParserErrors::InvalidOperand {
            loc: (start.0, start.1 - 1),
        })?,
    })
}

fn extend_operation(operation: Node, operator: Operator, node: Node) -> Result<Node, ParserErrors> {
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
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::operation_parser::{
        extend_operation, parse_operation, parse_operation_if_operator_follows,
    };

    #[test]
    fn test_parse_operation_if_operator_follows() {
        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.)
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
    fn test_simple_operations() {
        assert_eq!(
            parse_operation(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 3),
                        end: (0, 3),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
                true
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Subtract),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("7.5".into()),
                        start: (0, 2),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(5.),
                true
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(5.)),
                b: Box::new(Node::Number(7.5)),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(0.3),
                true
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(0.3)),
                b: Box::new(Node::Number(4.)),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Divide),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("1".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 3),
                        end: (0, 3),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(2.),
                true
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(2.)),
                b: Box::new(Node::Number(1.)),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Modulo),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("1.5".into()),
                        start: (0, 2),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(5.),
                true
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
        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Subtract),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 5),
                        end: (0, 5),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal(".5".into()),
                        start: (0, 2),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("7".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 8),
                        end: (0, 8),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal(".5".into()),
                        start: (0, 2),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Divide),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("7".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal(".5".into()),
                        start: (0, 2),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("7".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Divide),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal(".5".into()),
                        start: (0, 2),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("7".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("3".into()),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 8),
                        end: (0, 8),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(6.),
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

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(Token {
                        token: TokenType::Operator(Operator::Subtract),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5.5".into()),
                        start: (0, 3),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("8".into()),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Literal(".7".into()),
                        start: (0, 9),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 12),
                        end: (0, 12),
                    })
                ]
                .into_iter()
                .peekable(),
                Node::Number(0.2),
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
    fn test_extend_complex_operation() {
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
