mod temporaries;
mod transformers;

use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, StructureDeclaration},
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

impl From<&[StructureDeclaration]> for TypesRef {
    fn from(structures: &[StructureDeclaration]) -> Self {
        TypesRef(
            structures
                .iter()
                .map(|StructureDeclaration { name, fields }| {
                    (
                        name.clone(),
                        fields.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
                    )
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Void,
    Int,
    Char,
    Object(Box<str>),
}

impl<T: ToString> From<T> for Type {
    fn from(r#type: T) -> Self {
        match r#type.to_string().as_str() {
            "void" => Type::Void,
            "int" => Type::Int,
            "char" => Type::Char,
            r#type => Type::Object(r#type.into()),
        }
    }
}

impl From<Type> for Box<str> {
    fn from(val: Type) -> Self {
        match val {
            Type::Void => "void".into(),
            Type::Int => "int".into(),
            Type::Char => "char".into(),
            Type::Object(object) => object,
        }
    }
}

impl From<&Type> for Box<str> {
    fn from(val: &Type) -> Self {
        match val {
            Type::Void => "void".into(),
            Type::Int => "int".into(),
            Type::Char => "char".into(),
            Type::Object(object) => object.clone(),
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

pub fn transform(
    functions: &[FunctionDeclaration],
    structures: &[StructureDeclaration],
) -> Result<HashMap<Box<str>, Vec<Instruction>>, TransformerErrors> {
    let types_ref = structures.into();
    let functions_ref = functions.into();

    let mut funcs = HashMap::new();
    for FunctionDeclaration {
        body,
        return_type,
        parameters,
        name,
    } in functions
    {
        let mut b = Vec::new();
        let mut temporaries = Temporaries::default();
        let mut i = 0;
        for (parameter_name, parameter_type) in parameters.iter() {
            let parameter_type = parameter_type.into();
            if let Type::Object(object_type) = &parameter_type {
                for (field, field_type) in object_fields_recursive(&types_ref, object_type)? {
                    let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name, field));
                    temporaries.declare_named(field.clone(), field_type);
                    b.push(Instruction::LoadArgument(i, field));
                    i += 1;
                }
            } else {
                temporaries.declare_named(parameter_name.clone(), parameter_type);
                b.push(Instruction::LoadArgument(i, parameter_name.clone()));
                i += 1;
            }
        }

        for node in body.iter() {
            b.append(&mut transformers::transform_statement(
                (&functions_ref, &types_ref),
                node.clone(),
                &return_type.into(),
                &mut temporaries,
            )?)
        }

        funcs.insert(name.clone(), b);
    }

    Ok(funcs)
}
