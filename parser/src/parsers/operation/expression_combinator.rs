use nilang_types::nodes::expressions::{ExpressionNode, Operation, Operator};

use super::operation_extender::extend_operation;

pub(super) fn combine_expressions(
    preceding: ExpressionNode,
    operator: Operator,
    following: ExpressionNode,
) -> Result<ExpressionNode, ()> {
    Ok(match (preceding, following) {
        (ExpressionNode::Object { .. }, _) | (_, ExpressionNode::Object { .. }) => Err(())?,

        (_, ExpressionNode::Operation(_)) => {
            unreachable!("expressions are always evaluated left-to-right")
        }
        (ExpressionNode::Operation(a), b) => {
            ExpressionNode::Operation(extend_operation(a, operator, b))
        }

        (a, b) => ExpressionNode::Operation(Operation {
            operator,
            a: Box::new(a),
            b: Box::new(b),
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_expressions() {
        assert_eq!(
            combine_expressions(
                ExpressionNode::Object {
                    r#type: Default::default(),
                    fields: Default::default()
                },
                Default::default(),
                ExpressionNode::Object {
                    r#type: Default::default(),
                    fields: Default::default()
                }
            ),
            Err(())
        );

        assert_eq!(
            combine_expressions(
                ExpressionNode::Primitive(Default::default()),
                Default::default(),
                ExpressionNode::Parenthesis(Default::default())
            ),
            Ok(ExpressionNode::Operation(Operation {
                operator: Default::default(),
                a: Box::new(ExpressionNode::Primitive(Default::default())),
                b: Box::new(ExpressionNode::Parenthesis(Default::default()))
            }))
        );
    }
}
