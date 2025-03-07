use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, TypesRef};

use super::transform_expression;

pub fn transform_variable_declaration(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    name: Box<str>,
    r#type: Box<str>,
    node: ExpressionNode,
) -> Result<Vec<Instruction>, TransformerErrors> {
    temporaries.declare_named(name.clone(), r#type.clone());

    let instructions = transform_expression(context, temporaries, node, name.clone())?;

    temporaries.access(&name)?;
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut temporaries = Temporaries::default();
        let result = transform_variable_declaration(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            "a".into(),
            "int".into(),
            ExpressionNode::Number(10.),
        )
        .unwrap();
        assert_eq!(result, [Instruction::LoadNumber("a".into(), 10.)]);

        let result = transform_variable_declaration(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            "b".into(),
            "int".into(),
            ExpressionNode::VariableReference("a".into()),
        )
        .unwrap();
        assert_eq!(result, &[Instruction::Copy("b".into(), "a".into())]);

        let result = transform_variable_declaration(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            "c".into(),
            "int".into(),
            ExpressionNode::Object {
                r#type: "int".into(),
                fields: vec![("x".into(), ExpressionNode::Number(10.))]
                    .into_iter()
                    .collect(),
            },
        )
        .unwrap();
        assert_eq!(
            result,
            &[
                Instruction::Allocate("c".into(), "int".into()),
                Instruction::LoadNumber("c.x".into(), 10.)
            ]
        );
    }
}
