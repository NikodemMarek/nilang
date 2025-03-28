mod temporaries;
mod transformers;

use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, StatementNode, StructureDeclaration},
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
    let context = (&functions_ref, &types_ref);

    let mut funcs = HashMap::new();
    for FunctionDeclaration {
        body,
        return_type,
        parameters,
        name,
    } in functions
    {
        let mut temporaries = Temporaries::default();

        let parameters = transform_parameters(
            context,
            &mut temporaries,
            parameters
                .iter()
                .map(|(name, r#type)| (name.clone(), r#type.into()))
                .collect::<Vec<_>>()
                .as_slice(),
        )?;
        let body = transform_body(context, &mut temporaries, body, &return_type.into())?;

        funcs.insert(name.clone(), parameters.chain(body).collect());
    }

    Ok(funcs)
}

fn transform_body(
    context: (&FunctionsRef, &TypesRef),
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
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,
    parameters: &[(Box<str>, Type)],
) -> Result<Box<dyn Iterator<Item = Instruction>>, TransformerErrors> {
    let mut instructions = Vec::new();
    for (parameter_name, parameter_type) in parameters.iter() {
        let parameter_type = parameter_type.clone();
        if let Type::Object(object_type) = &parameter_type {
            for (field, field_type) in object_fields_recursive(context.1, object_type)? {
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
