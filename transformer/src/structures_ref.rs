use std::{
    collections::HashMap,
    iter::{empty, once},
};

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{StructureDeclaration, Type},
};

use crate::{temporaries::Temporaries, InstructionsIterator};

#[derive(Debug, Default)]
pub struct StructuresRef(
    HashMap<Box<str>, HashMap<Box<str>, Type>>,
    HashMap<Box<str>, HashMap<Box<str>, Type>>,
);

impl StructuresRef {
    pub fn get_fields(&self, structure_name: &str) -> Option<&HashMap<Box<str>, Type>> {
        self.0.get(structure_name)
    }

    pub fn get_fields_flattened(
        &self,
        object_type: &str,
    ) -> Result<&HashMap<Box<str>, Type>, TransformerErrors> {
        self.1
            .get(object_type)
            .ok_or_else(|| TransformerErrors::TypeNotFound {
                name: object_type.into(),
            })
    }
}

impl TryFrom<&[StructureDeclaration]> for StructuresRef {
    type Error = TransformerErrors;

    fn try_from(
        structures: &[StructureDeclaration],
    ) -> Result<StructuresRef, errors::TransformerErrors> {
        let nested_structures = structures
            .iter()
            .map(|StructureDeclaration { name, fields }| (name.clone(), fields.clone()))
            .collect::<HashMap<_, _>>();
        let flattened_structures = nested_structures
            .keys()
            .map(|structure_name| {
                object_fields_recursive(&nested_structures, structure_name)
                    .map(|fields| (structure_name.clone(), fields))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(StructuresRef(nested_structures, flattened_structures))
    }
}

fn object_fields_recursive(
    r: &HashMap<Box<str>, HashMap<Box<str>, Type>>,
    object_type: &str,
) -> Result<HashMap<Box<str>, Type>, TransformerErrors> {
    let fields_map = if let Some(fields) = r.get(object_type) {
        fields
    } else {
        return Err(TransformerErrors::TypeNotFound {
            name: object_type.into(),
        });
    };

    let mut fields = HashMap::new();
    for (field, field_type) in fields_map {
        if let Type::Object(field_type) = field_type {
            fields.extend(&mut object_fields_recursive(r, field_type)?.iter().map(
                |(subfield, r#type)| (format!("{}.{}", field, subfield).into(), r#type.clone()),
            ));
        } else {
            fields.insert(field.clone(), field_type.clone());
        }
    }

    Ok(fields)
}

type FromToType = (Box<str>, Box<str>, Type);
pub fn object_fields_from_to(
    context: &StructuresRef,

    source: Box<str>,
    destination: Box<str>,

    object_type: &str,
) -> Result<Vec<FromToType>, TransformerErrors> {
    let mut collect = context
        .get_fields_flattened(object_type)?
        .iter()
        .map(|(field, field_type)| {
            let destination_temporary = <Box<str>>::from(format!("{}.{}", destination, field));
            let source_temporary = <Box<str>>::from(format!("{}.{}", source, field));
            (destination_temporary, source_temporary, field_type.clone())
        })
        .collect::<Vec<_>>();

    collect.sort();
    Ok(collect)
}

pub fn copy_all_fields<'a>(
    context: &StructuresRef,
    temporaries: &'a Temporaries,

    source: Box<str>,
    destination: Box<str>,

    object_type: &Type,
) -> InstructionsIterator<'a> {
    let object_type = match object_type {
        Type::Object(object_type) => object_type,
        Type::Void => return Box::new(empty()),
        Type::Int | Type::Char => {
            return Box::new(once(Ok(Instruction::Copy(destination, source))));
        }
    };

    let Ok(object_fields_from_to) =
        object_fields_from_to(context, source, destination, object_type)
    else {
        return Box::new(once(Err(TransformerErrors::TypeNotFound {
            name: object_type.clone(),
        })));
    };

    let instructions = object_fields_from_to.into_iter().flat_map(
        |(destination_temporary, source_temporary, field_type)| {
            temporaries.declare_named(source_temporary.clone(), field_type);

            once(Ok(Instruction::Declare(destination_temporary.clone()))).chain(Ok::<
                Result<Instruction, TransformerErrors>,
                TransformerErrors,
            >(
                temporaries
                    .access(&source_temporary.clone())
                    .map(|_| Instruction::Copy(destination_temporary, source_temporary)),
            ))
        },
    );

    Box::new(instructions)
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use nilang_types::nodes::{StructureDeclaration, Type};

    use crate::structures_ref::{object_fields_recursive, StructuresRef};

    pub fn test_structures_ref() -> StructuresRef {
        StructuresRef::try_from(
            [
                StructureDeclaration {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), Type::Int), ("y".into(), Type::Int)]),
                },
                StructureDeclaration {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), Type::Object("Point".into())),
                        ("end".into(), Type::Object("Point".into())),
                    ]),
                },
                StructureDeclaration {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), Type::Char),
                        ("anchor".into(), Type::Object("Point".into())),
                    ]),
                },
            ]
            .as_ref(),
        )
        .unwrap()
    }

    #[test]
    fn test_object_fields_recursive() {
        let types_ref = test_structures_ref();

        assert_eq!(
            object_fields_recursive(&types_ref.0, "Rect").unwrap(),
            HashMap::from([
                ("start.x".into(), Type::Int),
                ("start.y".into(), Type::Int),
                ("end.x".into(), Type::Int),
                ("end.y".into(), Type::Int),
            ])
        );
    }
}
