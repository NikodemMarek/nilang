use std::iter::once;

use errors::{NilangError, TransformerErrors};
use nilang_types::{nodes::ExpressionNode, Localizable};

use crate::{Context, Instruction, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_function_call<'a>(
    context @ Context {
        functions,
        temporaries,
        ..
    }: &'a Context,

    name: Localizable<Box<str>>,
    arguments: Localizable<Box<[Localizable<ExpressionNode>]>>,

    result: Box<str>,
    r#type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    let Ok(function_parameters) = functions.get_parameters(&name) else {
        return Box::new(once(Err(NilangError {
            location: name.location,
            error: TransformerErrors::FunctionNotFound((*name).clone()).into(),
        })));
    };
    let mut function_parameters_iter = function_parameters.iter();

    let mut instructions = vec![];
    let mut arguments_names = vec![];

    for node in arguments.object.iter() {
        let Some((_, argument_type)) = function_parameters_iter.next() else {
            return Box::new(once(Err(NilangError {
                location: arguments.location,
                error: TransformerErrors::FunctionCallArgumentsMismatch {
                    name: (*name).clone(),
                    expected: function_parameters_iter.len() + 1,
                    got: arguments.len(),
                }
                .into(),
            })));
        };

        let argument_temporary = temporaries.declare((**argument_type).clone());
        instructions.push(Ok(Instruction::Declare(argument_temporary.clone())));
        instructions.append(
            &mut transform_expression(
                context,
                node.clone(),
                argument_temporary.clone(),
                argument_type,
            )
            .collect(),
        );

        if let Type::Object(object_type) = (**argument_type).clone() {
            let Some(fields) = context.structures.get_fields_flattened(&object_type) else {
                return Box::new(once(Err(NilangError {
                    location: node.location,
                    error: TransformerErrors::TypeNotFound(object_type).into(),
                })));
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
    }

    Box::new(
        instructions
            .into_iter()
            .chain(once(Ok(Instruction::FunctionCall(
                (*name).clone(),
                arguments_names.into(),
                if let Type::Void = **r#type {
                    None
                } else {
                    Some(result.clone())
                },
            )))),
    )
}
