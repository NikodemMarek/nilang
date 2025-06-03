use errors::{NilangError, ParserErrors};
use nilang_types::{
    nodes::{ExpressionNode, Operator},
    tokens::TokenType,
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_single_expression;

pub fn parse_operation_if_operator_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    node: L<ExpressionNode>,
) -> Result<L<ExpressionNode>, NilangError> {
    Ok(
        if let L {
            object: TokenType::Operator(_),
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
    node: L<ExpressionNode>,
) -> Result<L<ExpressionNode>, NilangError> {
    Ok(
        if let L {
            object: TokenType::Operator(_),
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
    preceeding: L<ExpressionNode>,
    rearrange: bool,
) -> Result<L<ExpressionNode>, NilangError> {
    let operator = tokens.assume_operator()?;

    Ok(match preceeding.object {
        ExpressionNode::Number(_) => {
            let b = parse_single_expression(tokens)?;
            L::new(
                Location::between(&preceeding.location, &b.location),
                ExpressionNode::Operation {
                    operator,
                    a: Box::new(preceeding),
                    b: Box::new(b),
                },
            )
        }
        ExpressionNode::FieldAccess { .. } => {
            let b = parse_single_expression(tokens)?;
            L::new(
                Location::between(&preceeding.location, &b.location),
                ExpressionNode::Operation {
                    operator,
                    a: Box::new(preceeding),
                    b: Box::new(b),
                },
            )
        }
        ExpressionNode::VariableReference(_) | ExpressionNode::FunctionCall { .. } => {
            let b = parse_single_expression(tokens)?;
            L::new(
                Location::between(&preceeding.location, &b.location),
                ExpressionNode::Operation {
                    operator,
                    a: Box::new(preceeding),
                    b: Box::new(b),
                },
            )
        }
        ExpressionNode::Operation { .. } => {
            let following = parse_single_expression(tokens)?;
            let location = Location::between(&preceeding.location, &following.location);
            L::new(
                location,
                if rearrange {
                    extend_operation(preceeding, operator, following)?
                } else {
                    ExpressionNode::Operation {
                        operator,
                        a: Box::new(preceeding),
                        b: Box::new(following),
                    }
                },
            )
        }
        ExpressionNode::Object { .. } | ExpressionNode::Char(_) | ExpressionNode::String(_) => {
            Err(NilangError {
                location: preceeding.location,
                error: ParserErrors::InvalidOperand.into(),
            })?
        }
    })
}

fn extend_operation(
    preceding: L<ExpressionNode>,
    operator: L<Operator>,
    succeeding: L<ExpressionNode>,
) -> Result<ExpressionNode, NilangError> {
    if let ExpressionNode::Operation {
        operator: preceding_operator,
        a: preceding_a,
        b: preceding_b,
    } = &preceding.object
    {
        Ok(match operator.object {
            Operator::Add | Operator::Subtract => match preceding_operator.object {
                Operator::Add | Operator::Subtract => ExpressionNode::Operation {
                    operator,
                    a: Box::new(preceding),
                    b: Box::new(succeeding),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    ExpressionNode::Operation {
                        operator,
                        a: Box::new(preceding),
                        b: Box::new(succeeding),
                    }
                }
            },
            Operator::Multiply | Operator::Divide | Operator::Modulo => {
                match preceding_operator.object {
                    Operator::Add | Operator::Subtract => ExpressionNode::Operation {
                        operator: preceding_operator.clone(),
                        a: preceding_a.clone(),
                        b: Box::new(L::new(
                            Location::between(&preceding_b.location, &succeeding.location),
                            ExpressionNode::Operation {
                                operator,
                                a: preceding_b.clone(),
                                b: Box::new(succeeding),
                            },
                        )),
                    },
                    Operator::Multiply | Operator::Divide | Operator::Modulo => {
                        ExpressionNode::Operation {
                            operator,
                            a: Box::new(preceding),
                            b: Box::new(succeeding),
                        }
                    }
                }
            }
        })
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
        tokens::TokenType,
        Localizable as L,
    };

    use crate::parsers::operation_parser::{
        extend_operation, parse_operation, parse_operation_if_operator_follows,
    };

    #[test]
    fn test_parse_operation_if_operator_follows() {
        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.))
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
            }
        );
    }

    #[test]
    fn test_simple_operations() {
        assert_eq!(
            parse_operation(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
                true
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Subtract),)),
                    Ok(L::irrelevant(TokenType::Literal("7.5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(5.)),
                true
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Subtract),
                a: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(7.5))),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("4".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(0.3)),
                true
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Multiply),
                a: Box::new(L::irrelevant(ExpressionNode::Number(0.3))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(4.))),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Divide),)),
                    Ok(L::irrelevant(TokenType::Literal("1".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(2.)),
                true
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Divide),
                a: Box::new(L::irrelevant(ExpressionNode::Number(2.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(1.))),
            }
        );

        assert_eq!(
            parse_operation(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Modulo),)),
                    Ok(L::irrelevant(TokenType::Literal("1.5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(5.)),
                true
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Modulo),
                a: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(1.5))),
            }
        );
    }

    #[test]
    fn parse_complex_operations() {
        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Subtract),)),
                    Ok(L::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Subtract),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal(".5".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("7".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Multiply),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(0.5))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(7.))),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal(".5".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Divide),)),
                    Ok(L::irrelevant(TokenType::Literal("7".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Divide),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(0.5))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(7.0))),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal(".5".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("7".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(0.5))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(7.))),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Divide),)),
                    Ok(L::irrelevant(TokenType::Literal(".5".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("7".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("3".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(6.)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Divide),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(0.5))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(7.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(3.))),
                })),
            }
        );

        assert_eq!(
            parse_operation_if_operator_follows(
                &mut [
                    Ok(L::irrelevant(TokenType::Operator(Operator::Subtract),)),
                    Ok(L::irrelevant(TokenType::Literal("5.5".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("8".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal(".7".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant(ExpressionNode::Number(0.2)),
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Subtract),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(0.2))),
                    b: Box::new(L::irrelevant(ExpressionNode::Operation {
                        operator: L::irrelevant(Operator::Multiply),
                        a: Box::new(L::irrelevant(ExpressionNode::Number(5.5))),
                        b: Box::new(L::irrelevant(ExpressionNode::Number(8.))),
                    })),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(0.7))),
            }
        );
    }

    #[test]
    fn test_extend_complex_operation() {
        assert_eq!(
            extend_operation(
                L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                }),
                L::irrelevant(Operator::Add),
                L::irrelevant(ExpressionNode::Number(4.))
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(4.)))
            }
        );

        assert_eq!(
            extend_operation(
                L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                }),
                L::irrelevant(Operator::Multiply),
                L::irrelevant(ExpressionNode::Number(4.))
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(8.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(4.)))
                }))
            }
        );

        assert_eq!(
            extend_operation(
                L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                }),
                L::irrelevant(Operator::Add),
                L::irrelevant(ExpressionNode::Number(4.))
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(4.)))
            }
        );

        assert_eq!(
            extend_operation(
                L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                }),
                L::irrelevant(Operator::Multiply),
                L::irrelevant(ExpressionNode::Number(4.))
            )
            .unwrap(),
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Multiply),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(8.)))
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(4.)))
            }
        );
    }
}
