use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{
    temporaries::Temporaries, FunctionsRef, Instruction, InstructionsIterator, StructuresRef, Type,
};

use super::transform_expression;

pub fn transform_return<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,

    node: ExpressionNode,

    return_type: &Type,
) -> InstructionsIterator<'a> {
    let variable_name = temporaries.declare(return_type.clone());
    let instructions = transform_expression(
        context,
        temporaries,
        node,
        variable_name.clone(),
        return_type,
    );

    let Ok(_) = temporaries.access(&variable_name) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: variable_name,
        })));
    };

    Box::new(
        once(Ok(Instruction::Declare(variable_name.clone())))
            .chain(instructions)
            .chain(once(Ok(Instruction::ReturnVariable(variable_name)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_return_variable() {
        let temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), Type::Int);

        assert_eq!(
            transform_return(
                &(FunctionsRef::default(), StructuresRef::default()),
                &temporaries,
                ExpressionNode::VariableReference("x".into()),
                &Type::Int,
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            [
                Instruction::Declare("temp_0".into()),
                Instruction::Copy("temp_0".into(), "x".into()),
                Instruction::ReturnVariable("temp_0".into())
            ]
        );
    }

    #[test]
    fn test_transform_return_field_access() {
        let temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), Type::Object("struct".into()));
        temporaries.declare_named("x.y".into(), Type::Int);

        assert_eq!(
            transform_return(
                &(FunctionsRef::default(), StructuresRef::default()),
                &temporaries,
                ExpressionNode::FieldAccess {
                    structure: Box::new(ExpressionNode::VariableReference("x".into())),
                    field: "y".into(),
                },
                &Type::Int,
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            [
                Instruction::Declare("temp_0".into()),
                Instruction::Copy("temp_0".into(), "x.y".into()),
                Instruction::ReturnVariable("temp_0".into())
            ]
        );
    }
}
