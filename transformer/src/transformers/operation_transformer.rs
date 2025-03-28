use std::iter::once;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{ExpressionNode, Operator},
};

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::transform_expression;

pub fn transform_operation(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    operator: Operator,
    a: ExpressionNode,
    b: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    if *r#type != Type::Int {
        return Err(TransformerErrors::TypeMismatch {
            expected: "int".into(),
            found: r#type.into(),
        });
    }

    let a_temporary = temporaries.declare("int".into());
    let a_instructions =
        transform_expression(context, temporaries, a, a_temporary.clone(), r#type)?;
    let b_temporary = temporaries.declare("int".into());
    let b_instructions =
        transform_expression(context, temporaries, b, b_temporary.clone(), r#type)?;

    temporaries.access(&a_temporary)?;
    temporaries.access(&b_temporary)?;
    Ok(Box::new(
        once(Instruction::Declare(a_temporary.clone()))
            .chain(a_instructions)
            .chain(once(Instruction::Declare(b_temporary.clone())))
            .chain(b_instructions)
            .chain(once(match operator {
                Operator::Add => Instruction::AddVariables(result, a_temporary, b_temporary),
                Operator::Subtract => {
                    Instruction::SubtractVariables(result, a_temporary, b_temporary)
                }
                Operator::Multiply => {
                    Instruction::MultiplyVariables(result, a_temporary, b_temporary)
                }
                Operator::Divide => Instruction::DivideVariables(result, a_temporary, b_temporary),
                Operator::Modulo => Instruction::ModuloVariables(result, a_temporary, b_temporary),
            })),
    ))
}
