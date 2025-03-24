use errors::TransformerErrors;
use nilang_types::instructions::Instruction;

use crate::{temporaries::Temporaries, FunctionsRef, Type, TypesRef};

pub fn transform_variable_reference(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    variable: Box<str>,
    (result_temporary_id, _): (Box<str>, &Type),
) -> Result<Vec<Instruction>, TransformerErrors> {
    let source_type = temporaries.type_of(&variable)?.to_owned();

    copy_all_fields(
        context,
        temporaries,
        variable,
        result_temporary_id,
        &source_type,
    )
}

pub fn copy_all_fields(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,
    source: Box<str>,
    destination: Box<str>,
    object_type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    if let Type::Object(_) = object_type {
        let mut instructions = Vec::new();
        for (temporary, temporary_type) in object_fields_recursive(context.1, object_type)?.unwrap()
        {
            let destination_temporary = <Box<str>>::from(format!("{}.{}", destination, temporary));
            let source_temporary = <Box<str>>::from(format!("{}.{}", source, temporary));
            temporaries.declare_named(source_temporary.clone(), temporary_type);
            instructions.push(Instruction::Copy(destination_temporary, source_temporary));
        }
        dbg!(&instructions);
        return Ok(instructions);
    }

    Ok(vec![Instruction::Copy(destination, source)])
}

pub fn object_fields_recursive(
    context: &TypesRef,

    object_type: &Type,
) -> Result<Option<Vec<(String, Type)>>, TransformerErrors> {
    let object_type = if let Type::Object(object_type) = object_type {
        object_type
    } else {
        return Ok(None);
    };

    let fields_map = if let Some(fields) = context.get_fields(&object_type) {
        fields
    } else {
        return Ok(None);
    };

    let mut fields = Vec::new();
    for (field, field_type) in fields_map {
        let v = object_fields_recursive(context, field_type)?;
        if let Some(v) = v {
            fields.append(
                &mut v
                    .iter()
                    .map(|(subfield, r#type)| (format!("{}.{}", field, subfield), r#type.clone()))
                    .collect(),
            );
        } else {
            fields.push((field.to_string(), field_type.clone()));
        }
    }

    fields.sort();
    Ok(Some(fields))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nilang_types::{instructions::Instruction, nodes::Structure};

    use crate::{
        temporaries::Temporaries,
        transformers::variable_reference_transformer::{
            object_fields_recursive, transform_variable_reference,
        },
        FunctionsRef, Type, TypesRef,
    };

    #[test]
    fn test_object_fields_recursive() {
        let types_ref = TypesRef::from(HashMap::from([
            (
                "Point".into(),
                Structure {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), "int".into()), ("y".into(), "int".into())]),
                },
            ),
            (
                "Rect".into(),
                Structure {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), "Point".into()),
                        ("end".into(), "Point".into()),
                    ]),
                },
            ),
            (
                "Label".into(),
                Structure {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), "str".into()),
                        ("anchor".into(), "Point".into()),
                    ]),
                },
            ),
        ]));

        let mut result = object_fields_recursive(&types_ref, &Type::Object("Rect".into()))
            .unwrap()
            .unwrap();
        result.sort();
        assert_eq!(
            result,
            [
                ("end.x".to_string(), "int".into()),
                ("end.y".to_string(), "int".into()),
                ("start.x".to_string(), "int".into()),
                ("start.y".to_string(), "int".into()),
            ],
        );

        let result = object_fields_recursive(&types_ref, &Type::Object("Label".into()))
            .unwrap()
            .unwrap();
        assert_eq!(
            result,
            [
                ("anchor.x".to_string(), "int".into()),
                ("anchor.y".to_string(), "int".into()),
                ("text".to_string(), "str".into()),
            ],
        );
    }

    #[test]
    fn test_transform_variable_reference() {
        let types_ref = TypesRef::from(HashMap::from([
            (
                "Point".into(),
                Structure {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), "int".into()), ("y".into(), "int".into())]),
                },
            ),
            (
                "Rect".into(),
                Structure {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), "Point".into()),
                        ("end".into(), "Point".into()),
                    ]),
                },
            ),
            (
                "Label".into(),
                Structure {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), "str".into()),
                        ("anchor".into(), "Point".into()),
                    ]),
                },
            ),
        ]));

        let mut temporaries = Temporaries::default();
        temporaries.declare_named("original".into(), "Rect".into());
        temporaries.declare_named("copy".into(), "Rect".into());

        let instructions = transform_variable_reference(
            (&FunctionsRef::default(), &types_ref),
            &mut temporaries,
            "original".into(),
            ("copy".into(), &Type::Int),
        )
        .unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::Copy("copy.end.x".into(), "original.end.x".into()),
                Instruction::Copy("copy.end.y".into(), "original.end.y".into()),
                Instruction::Copy("copy.start.x".into(), "original.start.x".into()),
                Instruction::Copy("copy.start.y".into(), "original.start.y".into()),
            ],
        );
    }
}
