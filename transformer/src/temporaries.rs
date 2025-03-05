use std::collections::HashMap;

use errors::TransformerErrors;

#[derive(Debug, Default)]
pub struct Temporaries(HashMap<Box<str>, (Box<str>, u8)>);

impl Temporaries {
    pub fn declare(&mut self, name: Box<str>, r#type: Box<str>) {
        self.0.insert(name, (r#type, 0));
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

    pub fn type_of(&self, name: &str) -> Result<&str, TransformerErrors> {
        match self.0.get(name) {
            Some(r#type) => Ok(r#type.0.as_ref()),
            None => Err(TransformerErrors::TemporaryNotFound { name: name.into() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporaries() {
        let mut temporaries = Temporaries::default();

        temporaries.declare("a".into(), "int".into());
        temporaries.declare("b".into(), "char".into());

        assert_eq!(temporaries.type_of("a").unwrap(), "int");
        assert_eq!(temporaries.type_of("b").unwrap(), "char");

        assert!(temporaries.type_of("c").is_err());
    }
}
