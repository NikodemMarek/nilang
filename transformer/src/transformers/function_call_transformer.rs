use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction};

pub fn transform_function_call(
    context: &FunctionsRef,
    temporaries: &mut Temporaries,

    name: Box<str>,
    arguments: &[ExpressionNode],
    return_type: Box<str>,
) -> Result<(Vec<Instruction>, Box<str>), TransformerErrors> {
    let function_parameters = context.get_parameters(&name)?;
    let mut function_parameters = function_parameters.iter();

    let acc = (&mut Vec::new(), &mut Vec::new());
    let (arguments, instructions) = arguments.iter().fold(acc, |acc, node| match node {
        ExpressionNode::Number(number) => {
            // TODO: Handle too many arguments
            let (temp, _) = function_parameters.next().unwrap();

            acc.0.push(temp.clone());
            acc.1.push(Instruction::LoadNumber(*number, temp.clone()));

            acc
        }
        _ => unimplemented!(),
    });

    let result_temporary = <Box<str>>::from(format!("{}@function_return", name));
    temporaries.declare(result_temporary.clone(), return_type);

    instructions.push(Instruction::FunctionCall(
        name,
        arguments.iter().cloned().collect::<Box<[_]>>(),
        result_temporary.clone(),
    ));
    Ok((instructions.to_vec(), result_temporary))
}
