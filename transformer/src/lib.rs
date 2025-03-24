mod temporaries;
mod transformers;

use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, Parameter, Program, Structure},
};
use temporaries::Temporaries;

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
struct FunctionsRef(HashMap<Box<str>, (Box<str>, Box<[Parameter]>)>);

impl FunctionsRef {
    pub fn add_function(
        &mut self,
        FunctionDeclaration {
            name,
            parameters,
            return_type,
            ..
        }: FunctionDeclaration,
    ) {
        self.0.insert(name, (return_type, parameters));
    }

    pub fn get_parameters(&self, name: &str) -> Result<&[Parameter], TransformerErrors> {
        self.0
            .get(name)
            .map(|(_, parameters)| parameters.as_ref())
            .ok_or(TransformerErrors::FunctionNotFound { name: name.into() })
    }
}

pub fn transform(
    Program {
        structures,
        functions,
    }: Program,
) -> Result<HashMap<Box<str>, Vec<Instruction>>, TransformerErrors> {
    let types_ref = structures.into();

    let mut functions_ref = FunctionsRef(HashMap::new());
    for (_, function_declaration) in functions.iter() {
        functions_ref.add_function(function_declaration.clone());
    }

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
        for (i, (parameter_name, parameter_type)) in parameters.iter().enumerate() {
            temporaries.declare_named(parameter_name.clone(), parameter_type.into());
            body.push(Instruction::LoadArgument(i, parameter_name.clone()));
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
