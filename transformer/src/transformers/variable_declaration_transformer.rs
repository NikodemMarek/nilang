use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, StructuresRef, Type};

use super::transform_expression;

pub fn transform_variable_declaration(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &mut Temporaries,

    name: Box<str>,
    r#type: &Type,
    node: ExpressionNode,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>>> {
    temporaries.declare_named(name.clone(), r#type.clone());
    let Ok(_) = temporaries.access(&name) else {
        println!("Temporary not found: {}", name);
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: name.clone(),
        })));
    };

    Box::new(
        once(Ok(Instruction::Declare(name.clone()))).chain(transform_expression(
            context,
            temporaries,
            node,
            name,
            r#type,
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut temporaries = Temporaries::default();
        let result = transform_variable_declaration(
            (&FunctionsRef::default(), &StructuresRef::default()),
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
            (&FunctionsRef::default(), &StructuresRef::default()),
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
