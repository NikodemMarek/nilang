use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

use super::transform_expression;

pub fn transform_return(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    node: ExpressionNode,
    return_type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let variable_name = temporaries.declare(return_type.clone());
    let instructions = transform_expression(
        context,
        temporaries,
        node,
        (variable_name.clone(), return_type),
    )?;

    temporaries.access(&variable_name)?;
    Ok([
        instructions,
        vec![Instruction::ReturnVariable(variable_name)],
    ]
    .concat())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_return_number() {
        let node = ExpressionNode::Number(42.0);
        let result = transform_return(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut Temporaries::default(),
            node,
            &Type::Int,
        )
        .unwrap();
        assert_eq!(result, [Instruction::ReturnNumber(42.0)]);
    }

    #[test]
    fn test_transform_return_variable() {
        let mut temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), "int".into());
        let node = ExpressionNode::VariableReference("x".into());
        let result = transform_return(
            (&FunctionsRef::default(), &TypesRef::default()),
            &mut temporaries,
            node,
            &Type::Int,
        )
        .unwrap();
        assert_eq!(result, [Instruction::ReturnVariable("x".into())]);
    }

    #[test]
    fn test_transform_return_field_access() {
        let mut temporaries = Temporaries::default();
        temporaries.declare_named("x".into(), "struct".into());
        temporaries.declare_named("x.y".into(), "int".into());
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
        assert_eq!(result, [Instruction::ReturnVariable("x.y".into())]);
    }
}
