use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::variable_reference_transformer::copy_all_fields;

pub fn transform_field_access(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    structure: ExpressionNode,
    field: Box<str>,
    (result_temporary_id, result_type): (Box<str>, &Type),
) -> Result<Vec<Instruction>, TransformerErrors> {
    let flattened_field = flatten_field_access(structure, field)?;

    copy_all_fields(
        context,
        temporaries,
        flattened_field.into(),
        result_temporary_id,
        &result_type,
    )
}

fn flatten_field_access(
    structure: ExpressionNode,
    field: Box<str>,
) -> Result<String, TransformerErrors> {
    match structure {
        ExpressionNode::VariableReference(variable) => Ok(format!("{}.{}", variable, field)),
        ExpressionNode::FieldAccess {
            structure: st,
            field: fl,
        } => Ok(format!("{}.{}", flatten_field_access(*st, fl)?, field)),
        _ => unimplemented!(),
    }
}
