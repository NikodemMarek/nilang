mod temporaries;
mod transformers;

use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, StatementNode, StructureDeclaration, Type},
};
use temporaries::Temporaries;
use transformers::object_fields_recursive;

#[derive(Debug, Default)]
pub struct TypesRef(HashMap<Box<str>, HashMap<Box<str>, Type>>);

impl TypesRef {
    pub fn get_fields(&self, structure_name: &str) -> Option<&HashMap<Box<str>, Type>> {
        self.0.get(structure_name)
    }
}

impl From<&[StructureDeclaration]> for TypesRef {
    fn from(structures: &[StructureDeclaration]) -> Self {
        TypesRef(
            structures
                .iter()
                .map(|StructureDeclaration { name, fields }| (name.clone(), fields.clone()))
                .collect(),
        )
    }
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

pub fn transform_function(
    context: &(FunctionsRef, TypesRef),
    FunctionDeclaration {
        body,
        return_type,
        parameters,
        ..
    }: &FunctionDeclaration,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let mut temporaries = Temporaries::default();

    let parameters = transform_parameters(
        context,
        &mut temporaries,
        parameters
            .iter()
            .map(|(name, r#type)| (name.clone(), r#type.clone()))
            .collect::<Vec<_>>()
            .as_slice(),
    )?;
    let body = transform_body(context, &mut temporaries, body, return_type)?;

    Ok(Box::new(parameters.chain(body)))
}

fn transform_body(
    context: &(FunctionsRef, TypesRef),
    temporaries: &mut Temporaries,
    body: &[StatementNode],
    return_type: &Type,
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    Ok(Box::new(
        body.iter()
            .map(|node| {
                transformers::transform_statement(context, node.clone(), return_type, temporaries)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten(),
    ))
}

fn transform_parameters(
    context: &(FunctionsRef, TypesRef),
    temporaries: &mut Temporaries,
    parameters: &[(Box<str>, Type)],
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let mut instructions = Vec::new();
    for (parameter_name, parameter_type) in parameters.iter() {
        let parameter_type = parameter_type.clone();
        if let Type::Object(object_type) = &parameter_type {
            for (field, field_type) in object_fields_recursive(&context.1, object_type)? {
                let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name, field));
                temporaries.declare_named(field.clone(), field_type);
                instructions.push(Instruction::Declare(field.clone()));
            }
        } else {
            temporaries.declare_named(parameter_name.clone(), parameter_type);
            instructions.push(Instruction::Declare(parameter_name.clone()));
        }
    }
    Ok(Box::new(instructions.into_iter()))
}
