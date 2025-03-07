use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{ExpressionNode, Operator},
};

use crate::{temporaries::Temporaries, FunctionsRef, TypesRef};

use super::transform_expression;

pub fn transform_operation(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    result_temporary_id: Box<str>,
    operator: Operator,
    a: ExpressionNode,
    b: ExpressionNode,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let a_temporary = temporaries.declare("int".into());
    let a_instructions = transform_expression(context, temporaries, a, a_temporary.clone())?;
    let b_temporary = temporaries.declare("int".into());
    let b_instructions = transform_expression(context, temporaries, b, b_temporary.clone())?;

    temporaries.access(&a_temporary)?;
    temporaries.access(&b_temporary)?;
    Ok([
        a_instructions,
        b_instructions,
        match operator {
            Operator::Add => vec![Instruction::AddVariables(
                result_temporary_id,
                a_temporary,
                b_temporary,
            )],
            Operator::Subtract => vec![Instruction::SubtractVariables(
                result_temporary_id,
                a_temporary,
                b_temporary,
            )],
            Operator::Multiply => vec![Instruction::MultiplyVariables(
                result_temporary_id,
                a_temporary,
                b_temporary,
            )],
            Operator::Divide => vec![Instruction::DivideVariables(
                result_temporary_id,
                a_temporary,
                b_temporary,
            )],
            Operator::Modulo => vec![Instruction::ModuloVariables(
                result_temporary_id,
                a_temporary,
                b_temporary,
            )],
        },
    ]
    .concat())
}
