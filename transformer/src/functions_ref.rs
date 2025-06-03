use std::collections::HashMap;

use errors::TransformerErrors;
use nilang_types::{
    nodes::{FunctionDeclaration, Parameter, Type},
    Localizable,
};

#[derive(Debug, Default)]
pub struct FunctionsRef(HashMap<Box<str>, (Type, Box<[Parameter]>)>);

impl FunctionsRef {
    pub fn get_parameters(&self, name: &str) -> Result<&[Parameter], TransformerErrors> {
        self.0
            .get(name)
            .map(|(_, parameters)| parameters.as_ref())
            .ok_or(TransformerErrors::FunctionNotFound(name.into()))
    }
}

impl From<&[Localizable<FunctionDeclaration>]> for FunctionsRef {
    fn from(functions: &[Localizable<FunctionDeclaration>]) -> Self {
        fn parse_function(
            FunctionDeclaration {
                return_type,
                parameters,
                name,
                ..
            }: &FunctionDeclaration,
        ) -> (Box<str>, (Type, Box<[Parameter]>)) {
            (
                (**name).clone(),
                (
                    (**return_type).clone(),
                    parameters
                        .iter()
                        .map(|(name, r#type)| (name.clone(), r#type.clone()))
                        .collect(),
                ),
            )
        }

        let mut functions = FunctionsRef(functions.iter().map(|f| parse_function(f)).collect());

        functions.0.insert(
            "printi".into(),
            (
                Type::Void,
                Box::new([(
                    Localizable::irrelevant("value".into()),
                    Localizable::irrelevant(Type::Int),
                )]),
            ),
        );
        functions.0.insert(
            "printc".into(),
            (
                Type::Void,
                Box::new([(
                    Localizable::irrelevant("value".into()),
                    Localizable::irrelevant(Type::Char),
                )]),
            ),
        );
        functions.0.insert(
            "print".into(),
            (
                Type::Void,
                Box::new([(
                    Localizable::irrelevant("value".into()),
                    Localizable::irrelevant(Type::String),
                )]),
            ),
        );

        functions
    }
}
