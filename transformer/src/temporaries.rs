use std::collections::HashMap;

use errors::TransformerErrors;

#[derive(Debug, Default)]
pub struct Temporaries(HashMap<Box<str>, Box<str>>);

impl Temporaries {
    pub fn insert(&mut self, name: Box<str>, r#type: Box<str>) {
        self.0.insert(name, r#type);
    }

    pub fn type_of(&self, name: &str) -> Result<&str, TransformerErrors> {
        match self.0.get(name) {
            Some(r#type) => Ok(r#type.as_ref()),
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }
}
