use std::{
    collections::HashMap,
    iter::{empty, once},
};

use errors::{NilangError, TransformerErrors};
use nilang_types::{
    instructions::Instruction,
    nodes::{StructureDeclaration, Type},
    Localizable,
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

    pub fn get_fields_flattened(&self, object_type: &str) -> Option<&HashMap<Box<str>, Type>> {
        self.1.get(object_type)
    }
}

impl TryFrom<&[StructureDeclaration]> for StructuresRef {
    type Error = TransformerErrors;

    fn try_from(
        structures: &[StructureDeclaration],
    ) -> Result<StructuresRef, errors::TransformerErrors> {
        fn parse_structure_unnested(
            StructureDeclaration { name, fields }: &StructureDeclaration,
        ) -> (Box<str>, HashMap<Box<str>, Type>) {
            (
                (**name).clone(),
                HashMap::from_iter(fields.iter().map(|(field_name, field_type)| {
                    ((**field_name).clone(), (**field_type).clone())
                })),
            )
        }

        let nested_structures = structures
            .iter()
            .map(parse_structure_unnested)
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
    let Some(fields_map) = r.get(object_type) else {
        return Err(TransformerErrors::TypeNotFound(object_type.into()));
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
    let Some(flattened) = context.get_fields_flattened(object_type) else {
        return Err(TransformerErrors::TypeNotFound(object_type.into()));
    };

    let mut collect = flattened
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

    object_type: &Localizable<Type>,
) -> InstructionsIterator<'a> {
    let r#type = match (**object_type).clone() {
        Type::Object(object_type) => object_type,
        Type::Void => return Box::new(empty()),
        Type::Int | Type::Char | Type::String => {
            return Box::new(once(Ok(Instruction::Copy(destination, source))));
        }
    };

    let Ok(object_fields_from_to) = object_fields_from_to(context, source, destination, &r#type)
    else {
        return Box::new(once(Err(NilangError {
            location: object_type.location,
            error: TransformerErrors::TypeNotFound(r#type.clone()).into(),
        })));
    };

    let instructions = object_fields_from_to.into_iter().flat_map(
        |(destination_temporary, source_temporary, field_type)| {
            temporaries.declare_named(source_temporary.clone(), field_type);

            once(Ok(Instruction::Declare(destination_temporary.clone()))).chain(Ok::<
                Result<Instruction, NilangError>,
                NilangError,
            >(
                match temporaries.access(&source_temporary.clone()) {
                    Some(_) => Ok(Instruction::Copy(destination_temporary, source_temporary)),
                    None => unreachable!(),
                },
            ))
        },
    );

    Box::new(instructions)
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use nilang_types::{
        nodes::{StructureDeclaration, Type},
        Localizable,
    };

    use crate::structures_ref::{object_fields_recursive, StructuresRef};

    pub fn test_structures_ref() -> StructuresRef {
        StructuresRef::try_from(
            [
                StructureDeclaration {
                    name: Localizable::irrelevant("Point".into()),
                    fields: Localizable::irrelevant(HashMap::from([
                        (
                            Localizable::irrelevant("x".into()),
                            Localizable::irrelevant(Type::Int),
                        ),
                        (
                            Localizable::irrelevant("y".into()),
                            Localizable::irrelevant(Type::Int),
                        ),
                    ])),
                },
                StructureDeclaration {
                    name: Localizable::irrelevant("Rect".into()),
                    fields: Localizable::irrelevant(HashMap::from([
                        (
                            Localizable::irrelevant("start".into()),
                            Localizable::irrelevant(Type::Object("Point".into())),
                        ),
                        (
                            Localizable::irrelevant("end".into()),
                            Localizable::irrelevant(Type::Object("Point".into())),
                        ),
                    ])),
                },
                StructureDeclaration {
                    name: Localizable::irrelevant("Label".into()),
                    fields: Localizable::irrelevant(HashMap::from([
                        (
                            Localizable::irrelevant("text".into()),
                            Localizable::irrelevant(Type::Char),
                        ),
                        (
                            Localizable::irrelevant("anchor".into()),
                            Localizable::irrelevant(Type::Object("Point".into())),
                        ),
                    ])),
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
