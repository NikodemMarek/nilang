use nilang_types::{nodes::ExpressionNode, Localizable};

use crate::{structures_ref::copy_all_fields, Context, InstructionsIterator, Type};

pub fn transform_field_access<'a>(
    Context {
        structures,
        temporaries,
        ..
    }: &'a Context,

    structure: Localizable<ExpressionNode>,
    field: Localizable<Box<str>>,

    result: Box<str>,
    r#type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    let flattened_field = flatten_field_access((*structure).clone(), (*field).clone());
    copy_all_fields(
        structures,
        temporaries,
        flattened_field.into(),
        result,
        r#type,
    )
}

fn flatten_field_access(structure: ExpressionNode, field: Box<str>) -> String {
    match structure {
        ExpressionNode::VariableReference(variable) => format!("{}.{}", variable, field),
        ExpressionNode::FieldAccess {
            structure: st,
            field: fl,
        } => format!(
            "{}.{}",
            flatten_field_access((**st).clone(), (*fl).clone()),
            field
        ),
        _ => unimplemented!(),
    }
}
