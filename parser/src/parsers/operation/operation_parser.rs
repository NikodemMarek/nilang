use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator, Primitive};

use crate::parsers::operation::operation_extender::extend_operation;

pub(super) fn combine_expressions(
    preceeding: ExpressionNode,
    operator: Operator,
    following: ExpressionNode,
) -> Result<Operation, ()> {
    match preceeding {
        a @ ExpressionNode::Primitive(Primitive::Number(_))
        | a @ ExpressionNode::Parenthesis(_)
        | a @ ExpressionNode::FieldAccess { .. }
        | a @ ExpressionNode::VariableReference(_)
        | a @ ExpressionNode::FunctionCall { .. } => Ok(Operation {
            operator,
            a: Box::new(a),
            b: Box::new(following),
        }),
        ExpressionNode::Operation(a) => Ok(extend_operation(a, operator, following)),
        ExpressionNode::Object { .. } | ExpressionNode::Primitive(_) => Err(())?,
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator, Primitive};

    use crate::parsers::operation::operation_parser::combine_expressions;

    #[test]
    fn test_simple_operations() {
        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Primitive::Number(6.)),
                Operator::Add,
                ExpressionNode::Primitive(Primitive::Number(9.)),
            )
            .unwrap(),
            Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(6.))),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(9.))),
            }
        );

        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Primitive::Number(5.)),
                Operator::Subtract,
                ExpressionNode::Primitive(Primitive::Number(7.5)),
            )
            .unwrap(),
            Operation {
                operator: Operator::Subtract,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(5.))),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(7.5))),
            }
        );

        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Primitive::Number(0.3)),
                Operator::Multiply,
                ExpressionNode::Primitive(Primitive::Number(4.)),
            )
            .unwrap(),
            Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(0.3))),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(4.))),
            }
        );

        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Primitive::Number(2.)),
                Operator::Divide,
                ExpressionNode::Primitive(Primitive::Number(1.)),
            )
            .unwrap(),
            Operation {
                operator: Operator::Divide,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(2.))),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(1.))),
            }
        );

        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Primitive::Number(5.)),
                Operator::Modulo,
                ExpressionNode::Primitive(Primitive::Number(1.5)),
            )
            .unwrap(),
            Operation {
                operator: Operator::Modulo,
                a: Box::new(ExpressionNode::Primitive(Primitive::Number(5.))),
                b: Box::new(ExpressionNode::Primitive(Primitive::Number(1.5))),
            }
        );
    }
}
