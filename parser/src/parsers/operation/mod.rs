use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

mod operation_extender;
mod operation_parser;

pub fn lookup_operation_recursive<I: PeekableAssumingIterator>(
    tokens: &mut I,
    preceeding: ExpressionNode,
) -> Result<ExpressionNode, NilangError> {
    if let Token {
        token: TokenType::Operator(_),
        ..
    } = tokens.peek_valid()?
    {
        let (start, end, operator) = tokens.assume_operator()?;
        let following = super::parse_single_expression(tokens)?;

        let operation = operation_parser::combine_expressions(preceeding, operator, following)
            .map_err(|_| NilangError {
                location: CodeLocation::range(start.0, start.1, end.0, end.1),
                error: ParserErrors::InvalidOperand.into(),
            })?;

        lookup_operation_recursive(tokens, ExpressionNode::Operation(operation))
    } else {
        Ok(preceeding)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operation, Operator},
        tokens::{Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::operation::lookup_operation_recursive};

    #[test]
    fn test_parse_operation_if_operator_follows() {
        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.)
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                })),
                b: Box::new(ExpressionNode::Number(5.)),
            })
        );
    }

    #[test]
    fn parse_complex_operations() {
        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                })),
                b: Box::new(ExpressionNode::Number(5.)),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Subtract,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                })),
                b: Box::new(ExpressionNode::Number(5.)),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                })),
                b: Box::new(ExpressionNode::Number(7.)),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Divide,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                })),
                b: Box::new(ExpressionNode::Number(7.0)),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                })),
                b: Box::new(ExpressionNode::Number(7.)),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(6.),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Divide,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(0.5)),
                })),
                b: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(7.)),
                    b: Box::new(ExpressionNode::Number(3.)),
                })),
            })
        );

        assert_eq!(
            lookup_operation_recursive(
                &mut MultiPeekable::new(
                    [
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
                ),
                ExpressionNode::Number(0.2),
            )
            .unwrap(),
            ExpressionNode::Operation(Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Subtract,
                    a: Box::new(ExpressionNode::Number(0.2)),
                    b: Box::new(ExpressionNode::Operation(Operation {
                        operator: Operator::Multiply,
                        a: Box::new(ExpressionNode::Number(5.5)),
                        b: Box::new(ExpressionNode::Number(8.)),
                    })),
                })),
                b: Box::new(ExpressionNode::Number(0.7)),
            })
        );
    }
}
