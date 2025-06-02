use std::iter::once;

use nilang_types::{nodes::ExpressionNode, Localizable};

use crate::{Context, Instruction, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_variable_declaration<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    name: Localizable<Box<str>>,
    r#type: &Localizable<Type>,
    node: Localizable<ExpressionNode>,
) -> InstructionsIterator<'a> {
    temporaries.declare_named((*name).clone(), (**r#type).clone());

    assert!(temporaries.access(&name).is_some());

    Box::new(
        once(Ok(Instruction::Declare((*name).clone()))).chain(transform_expression(
            context,
            node,
            (*name).clone(),
            r#type,
        )),
    )
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        structures_ref::tests::test_structures_ref, temporaries::Temporaries, FunctionsRef,
    };

    use super::*;

    #[test]
    fn test_variable_declaration() {
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            data: &RefCell::new(Vec::new()),
        };

        assert_eq!(
            transform_variable_declaration(
                &context,
                Localizable::irrelevant("a".into()),
                &Localizable::irrelevant(Type::Int),
                Localizable::irrelevant(ExpressionNode::Number(10.)),
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
                Localizable::irrelevant("b".into()),
                &Localizable::irrelevant(Type::Int),
                Localizable::irrelevant(ExpressionNode::VariableReference("a".into())),
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
