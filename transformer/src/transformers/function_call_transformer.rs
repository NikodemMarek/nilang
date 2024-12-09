use errors::TransformerErrors;
use nilang_types::nodes::Node;

use crate::{temporaries::Temporaries, Instruction};

pub fn transform_function_call(
    temporaries: &mut Temporaries,

    name: Box<str>,
    arguments: &[Node],
) -> Result<Vec<Instruction>, TransformerErrors> {
    let acc = (&mut Vec::new(), &mut Vec::new());
    let (arguments, instructions) = arguments.iter().fold(acc, |acc, node| match node {
        Node::Number(number) => {
            let temp = <Box<str>>::from(format!("{}@argument", number));

            acc.0.push(temp.clone());
            acc.1.push(Instruction::LoadNumber(*number, temp));

            acc
        }
        _ => unimplemented!(),
    });

    let result_temporary = <Box<str>>::from(format!("{}@function_return", name));
    let return_type = temporaries.type_of(&format!("{}@function", name))?.into();
    temporaries.insert(result_temporary.clone(), return_type);

    instructions.push(Instruction::FunctionCall(
        name,
        arguments.iter().cloned().collect::<Box<[_]>>(),
        result_temporary.clone(),
    ));
    Ok(instructions.to_vec())
}
