use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

use super::{object_fields_recursive, transform_expression};

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
            instructions.push(Instruction::Declare(argument_temporary.clone()));
            instructions.append(&mut transform_expression(
                context,
                temporaries,
                node.clone(),
                argument_temporary.clone(),
                &argument_type.clone(),
            )?);

            if let Type::Object(object_type) = argument_type {
                arguments_names.append(
                    &mut object_fields_recursive(context.1, object_type)?
                        .iter()
                        .map(|(field, _)| format!("{}.{}", argument_temporary, field).into())
                        .collect(),
                );
            } else {
                arguments_names.push(argument_temporary);
            }
        } else {
            return Err(TransformerErrors::FunctionCallArgumentsMismatch {
                name,
                expected: function_parameters.len() + 1,
                got: arguments.len(),
            });
        }
    }

    instructions.push(Instruction::FunctionCall(
        name,
        arguments_names.into(),
        if let Type::Void = r#type {
            None
        } else {
            Some(result.clone())
        },
    ));
    Ok(instructions.to_vec())
}
