use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, StructuresRef, Type};

use super::transform_expression;

pub fn transform_return(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &mut Temporaries,

    node: ExpressionNode,

    return_type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>>> {
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
        let mut temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), Type::Int);
        let node = ExpressionNode::VariableReference("x".into());
        let result = transform_return(
            (&FunctionsRef::default(), &StructuresRef::default()),
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
            (&FunctionsRef::default(), &StructuresRef::default()),
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
