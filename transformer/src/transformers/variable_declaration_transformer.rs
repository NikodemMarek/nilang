use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

use super::transform_expression;

pub fn transform_variable_declaration(
    context: &(FunctionsRef, TypesRef),
    temporaries: &mut Temporaries,

    name: Box<str>,
    r#type: &Type,
    node: ExpressionNode,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    temporaries.declare_named(name.clone(), r#type.clone());
    temporaries.access(&name)?;

    Ok(Box::new(once(Instruction::Declare(name.clone())).chain(
        transform_expression(context, temporaries, node, name, r#type)?,
    )))
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
            &Type::Int,
            ExpressionNode::Number(10.),
        )
        .unwrap();
        assert_eq!(
            result,
            [
                Instruction::Declare("a".into()),
                Instruction::LoadNumber("a".into(), 10.)
            ]
        );

        let result = transform_variable_declaration(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            "b".into(),
            &Type::Int,
            ExpressionNode::VariableReference("a".into()),
        )
        .unwrap();
        assert_eq!(
            result,
            [
                Instruction::Declare("b".into()),
                Instruction::Copy("b".into(), "a".into())
            ]
        );
    }
}
