use std::{cell::RefCell, collections::HashMap};

use crate::Type;

type Declared = HashMap<Box<str>, (Type, u8)>;

#[derive(Debug, Default)]
pub struct Temporaries(RefCell<(Declared, usize)>);

impl Temporaries {
    pub fn declare_named(&self, name: Box<str>, r#type: Type) {
        self.0.borrow_mut().0.insert(name, (r#type, 0));
    }

    pub fn declare(&self, r#type: Type) -> Box<str> {
        let mut b = self.0.borrow_mut();
        let name = <Box<str>>::from(format!("temp_{}", b.1));
        b.1 += 1;
        b.0.insert(name.clone(), (r#type, 0));
        name
    }

    pub fn access(&self, name: &str) -> Option<Type> {
        self.0.borrow_mut().0.get_mut(name).map(|r#type| {
            r#type.1 += 1;
            r#type.0.clone()
        })
    }

    pub fn type_of(&self, name: &str) -> Option<Type> {
        self.0
            .borrow_mut()
            .0
            .get_mut(name)
            .map(|r#type| r#type.0.clone())
    }
}
