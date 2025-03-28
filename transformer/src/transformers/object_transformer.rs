use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::transform_expression;

pub fn transform_object(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    fields: HashMap<Box<str>, ExpressionNode>,

    result: Box<str>,
    r#type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let mut instructions = Vec::new();
    for (field, value) in fields.iter() {
        let field_temp = <Box<str>>::from(format!("{}.{}", result, field));
        temporaries.declare_named(field_temp.clone(), r#type.clone());

        instructions.push(Instruction::Declare(field_temp.clone()));
        instructions.append(
            &mut transform_expression(context, temporaries, value.clone(), field_temp, r#type)?
                .collect(),
        );
    }

    Ok(Box::new(instructions.into_iter()))
}
