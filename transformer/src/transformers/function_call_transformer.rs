use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, StructuresRef, Type};

use super::transform_expression;

pub fn transform_function_call(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &Temporaries,

    name: Box<str>,
    arguments: &[ExpressionNode],

    result: Box<str>,
    r#type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>>> {
    let Ok(function_parameters) = context.0.get_parameters(&name) else {
        return Box::new(once(Err(TransformerErrors::FunctionNotFound { name })));
    };
    let mut function_parameters = function_parameters.iter();

    let mut instructions = vec![];
    let mut arguments_names = vec![];

    for node in arguments {
        if let Some((_, argument_type)) = function_parameters.next() {
            let argument_temporary = temporaries.declare(argument_type.clone());
            instructions.push(Ok(Instruction::Declare(argument_temporary.clone())));
            instructions.append(
                &mut transform_expression(
                    context,
                    temporaries,
                    node.clone(),
                    argument_temporary.clone(),
                    &argument_type.clone(),
                )
                .collect(),
            );

            if let Type::Object(object_type) = argument_type {
                let fields = match context.1.get_fields_flattened(object_type) {
                    Ok(fields) => fields,
                    Err(e) => return Box::new(once(Err(e))),
                };

                arguments_names.append(
                    &mut fields
                        .iter()
                        .map(|(field, _)| format!("{}.{}", argument_temporary, field).into())
                        .collect(),
                );
            } else {
                arguments_names.push(argument_temporary);
            }
        } else {
            return Box::new(once(Err(
                TransformerErrors::FunctionCallArgumentsMismatch {
                    name,
                    expected: function_parameters.len() + 1,
                    got: arguments.len(),
                },
            )));
        }
    }

    Box::new(
        instructions
            .into_iter()
            .chain(once(Ok(Instruction::FunctionCall(
                name,
                arguments_names.into(),
                if let Type::Void = r#type {
                    None
                } else {
                    Some(result.clone())
                },
            )))),
    )
}
