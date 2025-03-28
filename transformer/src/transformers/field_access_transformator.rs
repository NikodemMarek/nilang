use errors::TransformerErrors;
use nilang_types::{instructions::Instruction, nodes::ExpressionNode};

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::copy_all_fields;

pub fn transform_field_access(
    context: &(FunctionsRef, TypesRef),
    temporaries: &mut Temporaries,

    structure: ExpressionNode,
    field: Box<str>,

    result: Box<str>,
    r#type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
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
