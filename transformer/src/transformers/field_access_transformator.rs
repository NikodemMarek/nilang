use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, StructuresRef, Type};

use super::copy_all_fields;

pub fn transform_field_access<'a>(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,

    structure: ExpressionNode,
    field: Box<str>,

    result: Box<str>,
    r#type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    let flattened_field = flatten_field_access(structure, field);
    copy_all_fields(context, temporaries, flattened_field.into(), result, r#type)
}

fn flatten_field_access(structure: ExpressionNode, field: Box<str>) -> String {
    match structure {
        ExpressionNode::VariableReference(variable) => format!("{}.{}", variable, field),
        ExpressionNode::FieldAccess {
            structure: st,
            field: fl,
        } => format!("{}.{}", flatten_field_access(*st, fl), field),
        _ => unimplemented!(),
    }
}
