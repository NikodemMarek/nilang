use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::nodes::{FunctionDeclaration, Parameter, Type};

#[derive(Debug, Default)]
pub struct FunctionsRef(HashMap<Box<str>, (Type, Box<[Parameter]>)>);

impl FunctionsRef {
    pub fn get_parameters(&self, name: &str) -> Result<&[Parameter], TransformerErrors> {
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
        functions.0.insert(
            "print".into(),
            (Type::Void, Box::new([("value".into(), Type::String)])),
        );

        functions
    }
}
