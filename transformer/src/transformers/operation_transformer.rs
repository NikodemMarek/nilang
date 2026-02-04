use std::iter::once;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::expressions::{Arithmetic, Boolean, Operation, Operator},
};

use crate::{Context, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_operation<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    Operation { operator, a, b }: Operation,

    result: Box<str>,
    r#type: &Type,
) -> InstructionsIterator<'a> {
    if *r#type != Type::Int {
        return Box::new(once(Err(TransformerErrors::TypeMismatch {
            expected: Type::Int,
            found: r#type.clone(),
        })));
    }

    let a_temporary = temporaries.declare(r#type.clone());
    let a_instructions = transform_expression(context, *a, a_temporary.clone(), r#type);
    let b_temporary = temporaries.declare(r#type.clone());
    let b_instructions = transform_expression(context, *b, b_temporary.clone(), r#type);

    let Ok(_) = temporaries.access(&a_temporary) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: a_temporary,
        })));
    };
    let Ok(_) = temporaries.access(&b_temporary) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: b_temporary,
        })));
    };

    let a_temporary_declare_copy = a_temporary.clone();
    let b_temporary_declare_copy = b_temporary.clone();

    let operator_instruction = match operator {
        Operator::Arithmetic(operator) => match operator {
            Arithmetic::Add => Instruction::AddVariables(result, a_temporary, b_temporary),
            Arithmetic::Subtract => {
                Instruction::SubtractVariables(result, a_temporary, b_temporary)
            }
            Arithmetic::Multiply => {
                Instruction::MultiplyVariables(result, a_temporary, b_temporary)
            }
            Arithmetic::Divide => Instruction::DivideVariables(result, a_temporary, b_temporary),
            Arithmetic::Modulo => Instruction::ModuloVariables(result, a_temporary, b_temporary),
        },
        Operator::Boolean(operator) => match operator {
            Boolean::Equal => Instruction::TestEqual(result, a_temporary, b_temporary),
            Boolean::NotEqual => Instruction::TestNotEqual(result, a_temporary, b_temporary),
            Boolean::Less => Instruction::TestLess(result, a_temporary, b_temporary),
            Boolean::More => Instruction::TestMore(result, a_temporary, b_temporary),
            Boolean::LessOrEqual => Instruction::TestLessOrEqual(result, a_temporary, b_temporary),
            Boolean::MoreOrEqual => Instruction::TestMoreOrEqual(result, a_temporary, b_temporary),
        },
    };

    Box::new(
        once(Ok(Instruction::Declare(a_temporary_declare_copy)))
            .chain(a_instructions)
            .chain(once(Ok(Instruction::Declare(b_temporary_declare_copy))))
            .chain(b_instructions)
            .chain(once(Ok(operator_instruction))),
    )
}
