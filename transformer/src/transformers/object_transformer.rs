use std::{
    collections::HashMap,
    iter::{once, zip},
};

use errors::{NilangError, TransformerErrors};
use nilang_types::{instructions::Instruction, nodes::ExpressionNode, Localizable};

use crate::{Context, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_object<'a>(
    context @ Context {
        structures,
        temporaries,
        ..
    }: &'a Context,

    fields: Localizable<HashMap<Localizable<Box<str>>, Localizable<ExpressionNode>>>,

    result: Box<str>,
    r#type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    let Type::Object(object_type) = (**r#type).clone() else {
        return Box::new(once(Err(NilangError {
            location: r#type.location,
            error: TransformerErrors::TypeMismatch {
                expected: (**r#type).clone(),
                found: Type::Int,
            }
            .into(),
        })));
    };

    let Some(object_fields) = structures.get_fields_flattened(&object_type) else {
        return Box::new(once(Err(NilangError {
            location: r#type.location,
            error: TransformerErrors::TypeNotFound(object_type.clone()).into(),
        })));
    };

    if fields.len() != object_fields.len() {
        return Box::new(once(Err(NilangError {
            location: fields.location,
            error: TransformerErrors::FieldsMismatch {
                expected: object_fields.keys().cloned().collect(),
                found: (*fields).keys().map(|k| (**k).clone()).collect(),
            }
            .into(),
        })));
    }

    let mut object_fields = object_fields.iter().collect::<Vec<_>>();
    object_fields.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut provided_fields = (*fields).clone().into_iter().collect::<Vec<_>>();
    provided_fields.sort_by(|(a, _), (b, _)| a.cmp(b));

    let instructions = zip(object_fields, provided_fields)
        .map(|((field, object_type), (_, value))| (field, object_type, value))
        .flat_map(move |(field, object_type, value)| {
            let field_temp = <Box<str>>::from(format!("{}.{}", result, field));
            temporaries.declare_named(field_temp.clone(), object_type.clone());

            let localizable_type = Localizable::new(value.location, object_type.clone());
            let expression = transform_expression(
                context,
                value.clone(),
                field_temp.clone(),
                &localizable_type,
            );
            once(Ok(Instruction::Declare(field_temp))).chain(expression)
        });

    Box::new(instructions)
}
