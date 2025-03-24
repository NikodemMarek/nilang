use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

use super::{transform_expression, variable_reference_transformer::object_fields_recursive};

pub fn transform_function_call(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    name: Box<str>,
    arguments: &[ExpressionNode],

    result: Box<str>,
    r#type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let function_parameters = context.0.get_parameters(&name)?;
    let mut function_parameters = function_parameters.iter();

    let mut instructions = vec![];
    let mut arguments_names = vec![];

    for node in arguments {
        if let Some((_, argument_type)) = function_parameters.next() {
            let argument_temporary = temporaries.declare(argument_type.clone());
            instructions.append(&mut copy_argument(
                context,
                temporaries,
                node.clone(),
                argument_temporary.clone(),
                &argument_type.clone(),
            )?);

            arguments_names.append(
                &mut object_fields_recursive(context.1, argument_type)?
                    .unwrap()
                    .iter()
                    .map(|(field, _)| format!("{}.{}", argument_temporary, field).into())
                    .collect(),
            );
        } else {
            panic!("Too many arguments");
        }
    }

    instructions.push(Instruction::FunctionCall(
        name,
        arguments_names.into(),
        result.clone(),
    ));
    Ok(instructions.to_vec())
}

fn copy_argument(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    argument: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let instructions = transform_expression(context, temporaries, argument, result, r#type)?;

    Ok(instructions)
}
