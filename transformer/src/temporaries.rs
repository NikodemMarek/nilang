use std::collections::HashMap;

use errors::TransformerErrors;

use crate::Type;

#[derive(Debug, Default)]
pub struct Temporaries(HashMap<Box<str>, (Type, u8)>, usize);

impl Temporaries {
    pub fn declare_named(&mut self, name: Box<str>, r#type: Type) {
        self.0.insert(name, (r#type, 0));
    }

    pub fn declare(&mut self, r#type: Type) -> Box<str> {
        let name = <Box<str>>::from(format!("temp_{}", self.1));
        self.1 += 1;
        self.declare_named(name.clone(), r#type);
        name
    }

    pub fn access(&mut self, name: &str) -> Result<&Type, TransformerErrors> {
        match self.0.get_mut(name) {
            Some(r#type) => {
                r#type.1 += 1;
                Ok(&r#type.0)
            }
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }

    pub fn type_of(&mut self, name: &str) -> Result<&Type, TransformerErrors> {
        match self.0.get_mut(name) {
            Some(r#type) => Ok(&r#type.0),
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }
}
