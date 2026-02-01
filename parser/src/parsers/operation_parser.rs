use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::{ExpressionNode, Operator},
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_single_expression;

pub fn parse_operation_if_operator_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    node: ExpressionNode,
) -> Result<ExpressionNode, NilangError> {
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
    node: ExpressionNode,
) -> Result<ExpressionNode, NilangError> {
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
    preceeding: ExpressionNode,
    rearrange: bool,
) -> Result<ExpressionNode, NilangError> {
    let (start, _, operator) = tokens.assume_operator()?;

    Ok(match preceeding {
        a @ ExpressionNode::Number(_) => ExpressionNode::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse_single_expression(tokens)?),
        },
        a @ ExpressionNode::FieldAccess { .. } => ExpressionNode::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(parse_single_expression(tokens)?),
        },
        a @ ExpressionNode::VariableReference(_) | a @ ExpressionNode::FunctionCall { .. } => {
            ExpressionNode::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(parse_single_expression(tokens)?),
            }
        }
        a @ ExpressionNode::Operation { .. } => {
            let following = parse_single_expression(tokens)?;
            if rearrange {
                extend_operation(a, operator, following)?
            } else {
                ExpressionNode::Operation {
                    operator,
                    a: Box::new(a),
                    b: Box::new(following),
                }
            }
        }
        ExpressionNode::Object { .. }
        | ExpressionNode::Boolean(_)
        | ExpressionNode::Char(_)
        | ExpressionNode::String(_) => Err(NilangError {
            location: CodeLocation::at(start.0, start.1 - 1),
            error: ParserErrors::InvalidOperand.into(),
        })?,
    })
}

fn extend_operation(
    operation: ExpressionNode,
    operator: Operator,
    node: ExpressionNode,
) -> Result<ExpressionNode, NilangError> {
    if let ExpressionNode::Operation {
        operator: prev_operator,
        a: prev_a,
        b: prev_b,
    } = operation
    {
        Ok(match operator {
            Operator::Add | Operator::Subtract => match prev_operator {
                Operator::Add | Operator::Subtract => ExpressionNode::Operation {
                    operator,
                    a: Box::new(ExpressionNode::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    ExpressionNode::Operation {
                        operator,
                        a: Box::new(ExpressionNode::Operation {
                            operator: prev_operator,
                            a: prev_a,
                            b: prev_b,
                        }),
                        b: Box::new(node),
                    }
                }
            },
            Operator::Multiply | Operator::Divide | Operator::Modulo => match prev_operator {
                Operator::Add | Operator::Subtract => ExpressionNode::Operation {
                    operator: prev_operator,
                    a: prev_a,
                    b: Box::new(ExpressionNode::Operation {
                        operator,
                        a: prev_b,
                        b: Box::new(node),
                    }),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    ExpressionNode::Operation {
                        operator,
                        a: Box::new(ExpressionNode::Operation {
                            operator: prev_operator,
                            a: prev_a,
                            b: prev_b,
                        }),
                        b: Box::new(node),
                    }
                }
            },
        })
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
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
                ExpressionNode::Number(6.)
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                }),
                b: Box::new(ExpressionNode::Number(5.)),
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
                ExpressionNode::Number(6.),
                true
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Number(6.)),
                b: Box::new(ExpressionNode::Number(9.)),
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
                ExpressionNode::Number(5.),
                true
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Subtract,
                a: Box::new(ExpressionNode::Number(5.)),
                b: Box::new(ExpressionNode::Number(7.5)),
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
                ExpressionNode::Number(0.3),
                true
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Number(0.3)),
                b: Box::new(ExpressionNode::Number(4.)),
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
                ExpressionNode::Number(2.),
                true
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Divide,
                a: Box::new(ExpressionNode::Number(2.)),
                b: Box::new(ExpressionNode::Number(1.)),
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
                ExpressionNode::Number(5.),
                true
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Modulo,
                a: Box::new(ExpressionNode::Number(5.)),
                b: Box::new(ExpressionNode::Number(1.5)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                }),
                b: Box::new(ExpressionNode::Number(5.)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Subtract,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                }),
                b: Box::new(ExpressionNode::Number(5.)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                }),
                b: Box::new(ExpressionNode::Number(7.)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Divide,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                }),
                b: Box::new(ExpressionNode::Number(7.0)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                }),
                b: Box::new(ExpressionNode::Number(7.)),
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
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Divide,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                }),
                b: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(7.)),
                    b: Box::new(ExpressionNode::Number(3.)),
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
                ExpressionNode::Number(0.2),
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Subtract,
                    a: Box::new(ExpressionNode::Number(0.2)),
                    b: Box::new(ExpressionNode::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(ExpressionNode::Number(5.5)),
                        b: Box::new(ExpressionNode::Number(8.)),
                    }),
                }),
                b: Box::new(ExpressionNode::Number(0.7)),
            }
        );
    }

    #[test]
    fn test_extend_complex_operation() {
        assert_eq!(
            extend_operation(
                ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Add,
                ExpressionNode::Number(4.)
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                }),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );

        assert_eq!(
            extend_operation(
                ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Multiply,
                ExpressionNode::Number(4.)
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Number(6.)),
                b: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(8.)),
                    b: Box::new(ExpressionNode::Number(4.))
                })
            }
        );

        assert_eq!(
            extend_operation(
                ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Add,
                ExpressionNode::Number(4.)
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                }),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );

        assert_eq!(
            extend_operation(
                ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Multiply,
                ExpressionNode::Number(4.)
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                }),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );
    }
}
