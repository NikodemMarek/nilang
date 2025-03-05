use errors::TransformerErrors;
use nilang_types::nodes::Node;

use crate::{temporaries::Temporaries, Instruction};

pub fn transform_function_call(
    context: &std::collections::HashMap<
        Box<str>,
        (
            Box<str>,
            std::collections::HashMap<Box<str>, Box<str>>,
            Vec<Node>,
        ),
    >,
    temporaries: &mut Temporaries,

    name: Box<str>,
    arguments: &[Node],
    return_type: Box<str>,
) -> Result<(Vec<Instruction>, Box<str>), TransformerErrors> {
    let mut function_parameters = context
        .get(&name)
        .ok_or(TransformerErrors::FunctionNotFound { name: name.clone() })?
        .1
        .keys();

    let acc = (&mut Vec::new(), &mut Vec::new());
    let (arguments, instructions) = arguments.iter().fold(acc, |acc, node| match node {
        Node::Number(number) => {
            let temp = function_parameters.next().unwrap().clone();

            acc.0.push(temp.clone());
            acc.1.push(Instruction::LoadNumber(*number, temp));

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
