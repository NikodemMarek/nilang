use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::transform_expression;

pub fn transform_object(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    r#type: &Type,
    fields: HashMap<Box<str>, ExpressionNode>,
    result_temporary_id: Box<str>,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let mut instructions = Vec::new();
    for (field, value) in fields.iter() {
        let field_temp = <Box<str>>::from(format!("{}.{}", result_temporary_id, field));
        temporaries.declare_named(field_temp.clone(), r#type.clone());

        let mut field_instructions =
            transform_expression(context, temporaries, value.clone(), (field_temp, r#type))?;
        instructions.append(&mut field_instructions);
    }
    Ok(instructions)
}
