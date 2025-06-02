use std::iter::once;

use nilang_types::{nodes::ExpressionNode, Localizable};

use crate::{Context, Instruction, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_return<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    node: Localizable<ExpressionNode>,

    return_type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    let variable_name = temporaries.declare((**return_type).clone());
    let instructions = transform_expression(context, node, variable_name.clone(), return_type);

    assert!(temporaries.access(&variable_name).is_some());

    Box::new(
        once(Ok(Instruction::Declare(variable_name.clone())))
            .chain(instructions)
            .chain(once(Ok(Instruction::ReturnVariable(variable_name)))),
    )
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use nilang_types::{
        instructions::Instruction,
        nodes::{ExpressionNode, Type},
        Localizable,
    };

    use crate::{
        structures_ref::tests::test_structures_ref, temporaries::Temporaries,
        transformers::return_transformer::transform_return, Context, FunctionsRef,
    };

    #[test]
    fn test_transform_return_variable() {
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            data: &RefCell::new(Vec::new()),
        };

        context.temporaries.declare_named("x".into(), Type::Int);

        assert_eq!(
            transform_return(
                &context,
                Localizable::irrelevant(ExpressionNode::VariableReference("x".into())),
                &Localizable::irrelevant(Type::Int),
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
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            data: &RefCell::new(Vec::new()),
        };

        context
            .temporaries
            .declare_named("x".into(), Type::Object("struct".into()));
        context.temporaries.declare_named("x.y".into(), Type::Int);

        assert_eq!(
            transform_return(
                &context,
                Localizable::irrelevant(ExpressionNode::FieldAccess {
                    structure: Box::new(Localizable::irrelevant(
                        ExpressionNode::VariableReference("x".into())
                    )),
                    field: Localizable::irrelevant("y".into()),
                }),
                &Localizable::irrelevant(Type::Int),
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
