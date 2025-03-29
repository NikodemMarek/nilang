use std::iter::once;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{ExpressionNode, Operator},
};

use crate::{temporaries::Temporaries, FunctionsRef, StructuresRef, Type};

use super::transform_expression;

pub fn transform_operation<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,

    operator: Operator,
    a: ExpressionNode,
    b: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    if *r#type != Type::Int {
        return Box::new(once(Err(TransformerErrors::TypeMismatch {
            expected: Type::Int,
            found: r#type.clone(),
        })));
    }

    let a_temporary = temporaries.declare(r#type.clone());
    let a_instructions = transform_expression(context, temporaries, a, a_temporary.clone(), r#type);
    let b_temporary = temporaries.declare(r#type.clone());
    let b_instructions = transform_expression(context, temporaries, b, b_temporary.clone(), r#type);

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

    Box::new(
        once(Ok(Instruction::Declare(a_temporary.clone())))
            .chain(a_instructions)
            .chain(once(Ok(Instruction::Declare(b_temporary.clone()))))
            .chain(b_instructions)
            .chain(once(Ok(match operator {
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
