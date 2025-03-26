mod temporaries;
mod transformers;

use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, Program, Structure},
};
use temporaries::Temporaries;
use transformers::object_fields_recursive;

#[derive(Debug, Default)]
struct TypesRef(HashMap<Box<str>, HashMap<Box<str>, Type>>);

impl TypesRef {
    pub fn get_fields(&self, structure_name: &str) -> Option<&HashMap<Box<str>, Type>> {
        self.0.get(structure_name)
    }
}

impl From<HashMap<Box<str>, Structure>> for TypesRef {
    fn from(structures: HashMap<Box<str>, Structure>) -> Self {
        TypesRef(
            structures
                .into_iter()
                .map(|(_, Structure { name, fields })| {
                    (
                        name,
                        fields.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
                    )
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Int,
    Object(Box<str>),
}

impl<T: ToString> From<T> for Type {
    fn from(r#type: T) -> Self {
        match r#type.to_string().as_str() {
            "int" => Type::Int,
            r#type => Type::Object(r#type.into()),
        }
    }
}

#[derive(Debug, Default)]
struct FunctionsRef(HashMap<Box<str>, (Type, Box<[(Box<str>, Type)]>)>);

impl FunctionsRef {
    pub fn get_parameters(&self, name: &str) -> Result<&[(Box<str>, Type)], TransformerErrors> {
        self.0
            .get(name)
            .map(|(_, parameters)| parameters.as_ref())
            .ok_or(TransformerErrors::FunctionNotFound { name: name.into() })
    }
}

impl From<HashMap<Box<str>, FunctionDeclaration>> for FunctionsRef {
    fn from(functions: HashMap<Box<str>, FunctionDeclaration>) -> Self {
        FunctionsRef(
            functions
                .into_iter()
                .map(
                    |(
                        name,
                        FunctionDeclaration {
                            return_type,
                            parameters,
                            ..
                        },
                    )| {
                        (
                            name,
                            (
                                return_type.into(),
                                parameters
                                    .iter()
                                    .map(|(name, r#type)| (name.clone(), r#type.into()))
                                    .collect(),
                            ),
                        )
                    },
                )
                .collect(),
        )
    }
}

pub fn transform(
    Program {
        structures,
        functions,
    }: Program,
) -> Result<HashMap<Box<str>, Vec<Instruction>>, TransformerErrors> {
    let types_ref = structures.into();
    let functions_ref = functions.clone().into();

    let mut functions_raw_body = HashMap::new();
    for (function_name, function_declaration) in functions {
        let nilang_types::nodes::FunctionDeclaration {
            body,
            return_type,
            parameters,
            ..
        } = function_declaration;

        functions_raw_body.insert(function_name, (return_type, parameters, body));
    }

    let mut funcs = HashMap::new();
    for (function_name, (return_type, parameters, function_body)) in
        functions_raw_body.clone().iter()
    {
        let mut body = Vec::new();
        let mut temporaries = Temporaries::default();
        let mut i = 0;
        for (parameter_name, parameter_type) in parameters.iter() {
            let parameter_type: Type = parameter_type.into();
            if let Type::Object(object_type) = &parameter_type {
                for (field, field_type) in object_fields_recursive(&types_ref, object_type)? {
                    let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name, field));
                    temporaries.declare_named(field.clone(), field_type);
                    body.push(Instruction::LoadArgument(i, field));
                    i += 1;
                }
            } else {
                temporaries.declare_named(parameter_name.clone(), parameter_type);
                body.push(Instruction::LoadArgument(i, parameter_name.clone()));
                i += 1;
            }
        }

        for node in function_body.iter() {
            body.append(&mut transformers::transform_statement(
                (&functions_ref, &types_ref),
                node.clone(),
                &return_type.into(),
                &mut temporaries,
            )?)
        }

        funcs.insert(function_name.clone(), body);
    }

    Ok(funcs)
}
