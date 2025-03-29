mod temporaries;
mod transformers;

use std::{collections::HashMap, iter::once};

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, StatementNode, StructureDeclaration, Type},
};
use temporaries::Temporaries;

#[derive(Debug, Default)]
pub struct StructuresRef(
    HashMap<Box<str>, HashMap<Box<str>, Type>>,
    HashMap<Box<str>, HashMap<Box<str>, Type>>,
);

impl StructuresRef {
    pub fn get_fields(&self, structure_name: &str) -> Option<&HashMap<Box<str>, Type>> {
        self.0.get(structure_name)
    }

    fn get_fields_flattened(
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

#[derive(Debug, Default)]
pub struct FunctionsRef(HashMap<Box<str>, (Type, Box<[(Box<str>, Type)]>)>);

impl FunctionsRef {
    pub fn get_parameters(&self, name: &str) -> Result<&[(Box<str>, Type)], TransformerErrors> {
        self.0
            .get(name)
            .map(|(_, parameters)| parameters.as_ref())
            .ok_or(TransformerErrors::FunctionNotFound { name: name.into() })
    }
}

impl From<&[FunctionDeclaration]> for FunctionsRef {
    fn from(functions: &[FunctionDeclaration]) -> Self {
        let mut functions = FunctionsRef(
            functions
                .iter()
                .map(
                    |FunctionDeclaration {
                         return_type,
                         parameters,
                         name,
                         ..
                     }| {
                        (
                            name.clone(),
                            (
                                return_type.clone(),
                                parameters
                                    .iter()
                                    .map(|(name, r#type)| (name.clone(), r#type.clone()))
                                    .collect(),
                            ),
                        )
                    },
                )
                .collect(),
        );

        functions.0.insert(
            "printi".into(),
            (Type::Void, Box::new([("value".into(), Type::Int)])),
        );
        functions.0.insert(
            "printc".into(),
            (Type::Void, Box::new([("value".into(), Type::Char)])),
        );

        functions
    }
}

pub fn transform_function<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    FunctionDeclaration {
        body,
        return_type,
        parameters,
        ..
    }: &'a FunctionDeclaration,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    let mut temporaries = Temporaries::default();

    let parameters = transform_parameters(
        &context.1,
        &mut temporaries,
        parameters
            .iter()
            .map(|(name, r#type)| (name.clone(), r#type.clone()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let body = transform_body(context, &mut temporaries, body, return_type);

    Box::new(parameters.chain(body).collect::<Vec<_>>().into_iter())
}

fn transform_body<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    temporaries: &'a mut Temporaries,
    body: &'a [StatementNode],
    return_type: &'a Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    Box::new(body.iter().flat_map(|node| {
        transformers::transform_statement(context, node.clone(), return_type, temporaries)
    }))
}

fn transform_parameters(
    context: &StructuresRef,
    temporaries: &mut Temporaries,
    parameters: &[(Box<str>, Type)],
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>>> {
    let mut instructions = Vec::new();
    let mut i = 0;
    for (parameter_name, parameter_type) in parameters.iter() {
        let parameter_type = parameter_type.clone();
        if let Type::Object(object_type) = &parameter_type {
            let object_fields_recursive = match context.get_fields_flattened(object_type) {
                Ok(object_fields_recursive) => object_fields_recursive,
                Err(e) => return Box::new(once(Err(e))),
            };

            for (field, field_type) in object_fields_recursive {
                let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name, field));
                temporaries.declare_named(field.clone(), field_type.to_owned());
                instructions.push(Ok(Instruction::TakeArgument(i, field.clone())));
                i += 1;
            }
        } else {
            temporaries.declare_named(parameter_name.clone(), parameter_type);
            instructions.push(Ok(Instruction::TakeArgument(i, parameter_name.clone())));
            i += 1;
        }
    }
    Box::new(instructions.into_iter())
}
