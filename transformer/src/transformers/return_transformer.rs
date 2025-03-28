use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

use super::transform_expression;

pub fn transform_return(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    node: ExpressionNode,

    return_type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let variable_name = temporaries.declare(return_type.clone());
    let instructions = transform_expression(
        context,
        temporaries,
        node,
        variable_name.clone(),
        return_type,
    )?;

    temporaries.access(&variable_name)?;
    Ok(Box::new(
        once(Instruction::Declare(variable_name.clone()))
            .chain(instructions)
            .chain(once(Instruction::ReturnVariable(variable_name))),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_return_variable() {
        let mut temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), Type::Int);
        let node = ExpressionNode::VariableReference("x".into());
        let result = transform_return(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            node,
            &Type::Int,
        )
        .unwrap();
        assert_eq!(
            result,
            [
                Instruction::Declare("temp_0".into()),
                Instruction::Copy("temp_0".into(), "x".into()),
                Instruction::ReturnVariable("temp_0".into())
            ]
        );
    }

    #[test]
    fn test_transform_return_field_access() {
        let mut temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), Type::Object("struct".into()));
        temporaries.declare_named("x.y".into(), Type::Int);
        let node = ExpressionNode::FieldAccess {
            structure: Box::new(ExpressionNode::VariableReference("x".into())),
            field: "y".into(),
        };
        let result = transform_return(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            node,
            &Type::Int,
        )
        .unwrap();
        assert_eq!(
            result,
            [
                Instruction::Declare("temp_0".into()),
                Instruction::Copy("temp_0".into(), "x.y".into()),
                Instruction::ReturnVariable("temp_0".into())
            ]
        );
    }
}
