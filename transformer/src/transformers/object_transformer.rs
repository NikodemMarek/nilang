use std::{
    collections::HashMap,
    iter::{once, zip},
};

use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, StructuresRef, Type};

use super::transform_expression;

pub fn transform_object(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &mut Temporaries,

    fields: HashMap<Box<str>, ExpressionNode>,

    result: Box<str>,
    r#type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>>> {
    let Type::Object(r#type) = r#type else {
        return Box::new(once(Err(TransformerErrors::TypeMismatch {
            expected: r#type.clone(),
            found: r#type.clone(),
        })));
    };

    let object_fields = match context.1.get_fields_flattened(r#type) {
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
    let mut provided_fields = fields.iter().collect::<Vec<_>>();
    provided_fields.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut instructions = Vec::new();
    for (field, r#type, value) in zip(object_fields, provided_fields)
        .map(|((field, r#type), (_, value))| (field, r#type, value))
    {
        let field_temp = <Box<str>>::from(format!("{}.{}", result, field));

        temporaries.declare_named(field_temp.clone(), r#type.clone());

        instructions.push(Ok(Instruction::Declare(field_temp.clone())));
        instructions.append(
            &mut transform_expression(context, temporaries, value.clone(), field_temp, r#type)
                .collect(),
        );
    }

    Box::new(instructions.into_iter())
}
