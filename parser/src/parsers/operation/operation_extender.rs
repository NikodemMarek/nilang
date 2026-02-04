use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator};

use super::precendence::is_preceeding;

pub(super) fn extend_operation(
    preceding: Operation,
    operator: Operator,
    following: ExpressionNode,
) -> Operation {
    if !is_preceeding(preceding.operator, operator) {
        Operation {
            operator: preceding.operator,
            a: preceding.a,
            b: Box::new(ExpressionNode::Operation(Operation {
                operator,
                a: preceding.b,
                b: Box::new(following),
            })),
        }
    } else {
        Operation {
            operator,
            a: Box::new(ExpressionNode::Operation(preceding)),
            b: Box::new(following),
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::expressions::{Arithmetic, Primitive};

    use super::*;

    #[test]
    fn test_extend_operation() {
        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Arithmetic(Arithmetic::Multiply),
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(1.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(2.)))
                },
                Operator::Arithmetic(Arithmetic::Add),
                ExpressionNode::Primitive(Primitive::Number(3.))
            ),
            Operation {
                operator: Operator::Arithmetic(Arithmetic::Add),
                a: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Arithmetic(Arithmetic::Multiply),
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(1.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(2.))),
                })),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(3.))),
            }
        );

        assert_eq!(
            extend_operation(
                Operation {
                    operator: Operator::Arithmetic(Arithmetic::Add),
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(1.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(2.)))
                },
                Operator::Arithmetic(Arithmetic::Multiply),
                ExpressionNode::Primitive(Primitive::Number(3.))
            ),
            Operation {
                operator: Operator::Arithmetic(Arithmetic::Add),
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(1.))),
                b: Box::new(ExpressionNode::Operation(Operation {
                    operator: Operator::Arithmetic(Arithmetic::Multiply),
                    a: Box::new(ExpressionNode::Primitive(Primitive::Number(2.))),
                    b: Box::new(ExpressionNode::Primitive(Primitive::Number(3.))),
                })),
            }
        );
    }
}
