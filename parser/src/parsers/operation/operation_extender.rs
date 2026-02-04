use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator};

use super::precendence::is_preceeding;

pub(super) fn extend_operation(
    prev: Operation,
    operator: Operator,
    node: ExpressionNode,
) -> Operation {
    if is_preceeding(operator, prev.operator) {
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
    use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator, Primitive};

    use crate::parsers::operation::operation_extender::extend_operation;

    #[test]
    fn test_extend_complex_operation() {
        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                },
                Operator::Add,
                ExpressionNode::Primitive(Primitive::Number(4.))
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                })),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(4.)))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                },
                Operator::Multiply,
                ExpressionNode::Primitive(Primitive::Number(4.))
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                b: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(8.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(4.)))
                }))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                },
                Operator::Add,
                ExpressionNode::Primitive(Primitive::Number(4.))
            ),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                })),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(4.)))
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                },
                Operator::Multiply,
                ExpressionNode::Primitive(Primitive::Number(4.))
            ),
            Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(8.)))
                })),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(4.)))
            }
        );
    }
}
