use errors::TransformerErrors;
use nilang_types::instructions::Instruction;

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

use super::copy_all_fields;

pub fn transform_variable_reference(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    variable: Box<str>,
    result: Box<str>,
    r#type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let source_type = temporaries.type_of(&variable)?.to_owned();

    if r#type != &source_type {
        return Err(TransformerErrors::TypeMismatch {
            expected: r#type.clone(),
            found: source_type.into(),
        });
    }

    copy_all_fields(context, temporaries, variable, result, &source_type)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nilang_types::{instructions::Instruction, nodes::StructureDeclaration};

    use crate::{
        temporaries::Temporaries,
        transformers::variable_reference_transformer::transform_variable_reference, FunctionsRef,
        Type, TypesRef,
    };

    #[test]
    fn test_transform_variable_reference() {
        let types_ref = TypesRef::from(
            [
                StructureDeclaration {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), "int".into()), ("y".into(), "int".into())]),
                },
                StructureDeclaration {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), "Point".into()),
                        ("end".into(), "Point".into()),
                    ]),
                },
                StructureDeclaration {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), "str".into()),
                        ("anchor".into(), "Point".into()),
                    ]),
                },
            ]
            .as_ref(),
        );

        let mut temporaries = Temporaries::default();
        temporaries.declare_named("original".into(), "Rect".into());
        temporaries.declare_named("copy".into(), "Rect".into());

        let instructions = transform_variable_reference(
            (&FunctionsRef::default(), &types_ref),
            &mut temporaries,
            "original".into(),
            "copy".into(),
            &Type::Object("Rect".into()),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::Declare("copy.end.x".into()),
                Instruction::Copy("copy.end.x".into(), "original.end.x".into()),
                Instruction::Declare("copy.end.y".into()),
                Instruction::Copy("copy.end.y".into(), "original.end.y".into()),
                Instruction::Declare("copy.start.x".into()),
                Instruction::Copy("copy.start.x".into(), "original.start.x".into()),
                Instruction::Declare("copy.start.y".into()),
                Instruction::Copy("copy.start.y".into(), "original.start.y".into()),
            ],
        );
    }
}
