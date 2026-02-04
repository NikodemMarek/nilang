use std::{
    collections::HashMap,
    iter::{once, zip},
};

use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::expressions::ExpressionNode};

use crate::{Context, InstructionsIterator, Type};

use super::transform_expression;

pub fn transform_object<'a>(
    context @ Context {
        structures,
        temporaries,
        ..
    }: &'a Context,

    fields: HashMap<Box<str>, ExpressionNode>,

    result: Box<str>,
    r#type: &Type,
) -> InstructionsIterator<'a> {
    let Type::Object(r#type) = r#type else {
        return Box::new(once(Err(TransformerErrors::TypeMismatch {
            expected: r#type.clone(),
            found: r#type.clone(),
        })));
    };

    let object_fields = match structures.get_fields_flattened(r#type) {
        Ok(object_fields) => object_fields,
        Err(e) => return Box::new(once(Err(e))),
    };

    if fields.len() != object_fields.len() {
        return Box::new(once(Err(TransformerErrors::FieldsMismatch {
            expected: object_fields.keys().cloned().collect(),
            found: fields.keys().cloned().collect(),
        })));
    }

    let mut object_fields = object_fields.iter().collect::<Vec<_>>();
    object_fields.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut provided_fields = fields.into_iter().collect::<Vec<_>>();
    provided_fields.sort_by(|(a, _), (b, _)| a.cmp(b));

    let instructions = zip(object_fields, provided_fields)
        .map(|((field, r#type), (_, value))| (field, r#type, value))
        .flat_map(move |(field, r#type, value)| {
            let field_temp = <Box<str>>::from(format!("{}.{}", result, field));
            temporaries.declare_named(field_temp.clone(), r#type.clone());

            let expression =
                transform_expression(context, value.clone(), field_temp.clone(), r#type);
            once(Ok(Instruction::Declare(field_temp))).chain(expression)
        });

    Box::new(instructions)
}
