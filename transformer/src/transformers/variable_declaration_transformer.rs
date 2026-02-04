use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::expressions::ExpressionNode;

use crate::{Context, Instruction, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_variable_declaration<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    name: Box<str>,
    r#type: &Type,
    node: ExpressionNode,
) -> InstructionsIterator<'a> {
    temporaries.declare_named(name.clone(), r#type.clone());

    let Ok(_) = temporaries.access(&name) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: name.clone(),
        })));
    };

    Box::new(
        once(Ok(Instruction::Declare(name.clone())))
            .chain(transform_expression(context, node, name, r#type)),
    )
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        labels::Labels, structures_ref::tests::test_structures_ref, temporaries::Temporaries,
        FunctionsRef,
    };

    use super::*;

    #[test]
    fn test_variable_declaration() {
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            labels: Labels::default(),
            data: &RefCell::new(Vec::new()),
        };

        assert_eq!(
            transform_variable_declaration(
                &context,
                "a".into(),
                &Type::Int,
                ExpressionNode::Number(10.),
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            [
                Instruction::Declare("a".into()),
                Instruction::LoadNumber("a".into(), 10.)
            ]
        );

        assert_eq!(
            transform_variable_declaration(
                &context,
                "b".into(),
                &Type::Int,
                ExpressionNode::VariableReference("a".into()),
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            [
                Instruction::Declare("b".into()),
                Instruction::Copy("b".into(), "a".into())
            ]
        );
    }
}
