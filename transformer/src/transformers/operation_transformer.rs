use std::iter::once;

use errors::{NilangError, TransformerErrors};
use nilang_types::{
    instructions::Instruction,
    nodes::{ExpressionNode, Operator},
    Localizable,
};

use crate::{Context, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_operation<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    operator: Localizable<Operator>,
    a: Localizable<ExpressionNode>,
    b: Localizable<ExpressionNode>,

    result: Box<str>,
    r#type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    if **r#type != Type::Int {
        return Box::new(once(Err(NilangError {
            location: r#type.location,
            error: TransformerErrors::TypeMismatch {
                expected: Type::Int,
                found: (**r#type).clone(),
            }
            .into(),
        })));
    }

    let a_temporary = temporaries.declare((**r#type).clone());
    let a_instructions = transform_expression(context, a, a_temporary.clone(), r#type);
    let b_temporary = temporaries.declare((**r#type).clone());
    let b_instructions = transform_expression(context, b, b_temporary.clone(), r#type);

    assert!(temporaries.access(&a_temporary).is_some());
    assert!(temporaries.access(&b_temporary).is_some());

    Box::new(
        once(Ok(Instruction::Declare(a_temporary.clone())))
            .chain(a_instructions)
            .chain(once(Ok(Instruction::Declare(b_temporary.clone()))))
            .chain(b_instructions)
            .chain(once(Ok(match *operator {
                Operator::Add => Instruction::AddVariables(result, a_temporary, b_temporary),
                Operator::Subtract => {
                    Instruction::SubtractVariables(result, a_temporary, b_temporary)
                }
                Operator::Multiply => {
                    Instruction::MultiplyVariables(result, a_temporary, b_temporary)
                }
                Operator::Divide => Instruction::DivideVariables(result, a_temporary, b_temporary),
                Operator::Modulo => Instruction::ModuloVariables(result, a_temporary, b_temporary),
            }))),
    )
}
