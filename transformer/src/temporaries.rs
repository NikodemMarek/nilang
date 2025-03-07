use std::collections::HashMap;

use errors::TransformerErrors;

#[derive(Debug, Default)]
pub struct Temporaries(HashMap<Box<str>, (Box<str>, u8)>);

impl Temporaries {
    pub fn declare_named(&mut self, name: Box<str>, r#type: Box<str>) {
        self.0.insert(name, (r#type, 0));
    }

    pub fn declare(&mut self, r#type: Box<str>) -> Box<str> {
        let name = <Box<str>>::from(format!("temp_{}", self.0.len()));
        self.declare_named(name.clone(), r#type);
        name
    }

    pub fn access(&mut self, name: &str) -> Result<&str, TransformerErrors> {
        match self.0.get_mut(name) {
            Some(r#type) => {
                r#type.1 += 1;
                Ok(r#type.0.as_ref())
            }
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }

    pub fn type_of(&mut self, name: &str) -> Result<&str, TransformerErrors> {
        match self.0.get_mut(name) {
            Some(r#type) => Ok(r#type.0.as_ref()),
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }
}
