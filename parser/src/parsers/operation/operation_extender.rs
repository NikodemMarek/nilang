use nilang_types::nodes::{ExpressionNode, Operation, Operator};

pub(super) fn extend_operation(
    prev: Operation,
    operator: Operator,
    node: ExpressionNode,
) -> Operation {
    let is_prev_low = matches!(prev.operator, Operator::Add | Operator::Subtract);
    let is_curr_high = matches!(
        operator,
        Operator::Multiply | Operator::Divide | Operator::Modulo
    );

    if is_prev_low && is_curr_high {
        Operation {
            operator: prev.operator,
            a: prev.a,
            b: Box::new(ExpressionNode::Operation(Operation {
                operator,
                a: prev.b,
                b: Box::new(node),
            })),
        }
    } else {
        Operation {
            operator,
            a: Box::new(ExpressionNode::Operation(prev)),
            b: Box::new(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{ExpressionNode, Operation, Operator};

    use crate::parsers::operation::operation_extender::extend_operation;

    #[test]
    fn test_extend_complex_operation() {
        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Add,
                ExpressionNode::Number(4.)
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                })),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Multiply,
                ExpressionNode::Number(4.)
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Number(6.)),
                b: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(8.)),
                    b: Box::new(ExpressionNode::Number(4.))
                }))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Add,
                ExpressionNode::Number(4.)
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                })),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                },
                Operator::Multiply,
                ExpressionNode::Number(4.)
            ),
            Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(8.))
                })),
                b: Box::new(ExpressionNode::Number(4.))
            }
        );
    }
}
