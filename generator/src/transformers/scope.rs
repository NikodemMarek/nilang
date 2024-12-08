use std::{collections::HashMap, rc::Rc};

use errors::GeneratorErrors;
use nilang_types::nodes::Node;

use crate::{transformers::transform, TypesRef};

#[derive(Default)]
pub struct Scope {
    tsr: Rc<TypesRef>,
    variables: HashMap<Box<str>, u8>,
    allocated: u8,
}

impl Scope {
    pub fn new(tsr: Rc<TypesRef>) -> Self {
        Scope {
            tsr,
            variables: HashMap::new(),
            allocated: 0,
        }
    }

    pub fn inherit(scope: &Scope) -> Self {
        Scope {
            tsr: Rc::clone(&scope.tsr),
            variables: scope.variables.clone(),
            allocated: scope.allocated,
        }
    }

    pub fn insert(&mut self, name: &str, r#type: &str) -> eyre::Result<u8> {
        let name = name.to_owned().into();
        if self.variables.contains_key::<Box<str>>(&name) {
            Err(GeneratorErrors::VariableAlreadyExists { name: name.clone() })?
        }

        self.allocated += self.tsr.get_structure_size(r#type)?;

        self.variables.insert(name, self.allocated);
        Ok(self.allocated)
    }
    pub fn insert_unnamed(&mut self, r#type: &str) -> eyre::Result<u8> {
        self.allocated += self.tsr.get_structure_size(r#type)?;
        Ok(self.allocated)
    }

    pub fn get(&self, name: &str) -> eyre::Result<u8> {
        let offset = match self.variables.get(name) {
            Some(offset) => *offset,
            None => Err(GeneratorErrors::VariableDoesNotExist { name: name.into() })?,
        };

        Ok(offset)
    }
}

pub fn transform_scope(a: &Node, tr: &TypesRef, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::Scope(inner) = a {
        let mut scope = Scope::inherit(scope);

        let mut code = Vec::with_capacity(4096);
        for node in inner {
            code.append(&mut transform(node, tr, &mut scope)?);
        }

        Ok(code)
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::Node;

    use crate::{transformers::scope::transform_scope, TypesRef};

    #[test]
    fn scope_with_return() {
        assert_eq!(
            transform_scope(
                &Node::Scope(vec![Node::Return(Box::new(Node::Number(42.)))]),
                &TypesRef::default(),
                &mut super::Scope::default(),
            )
            .unwrap(),
            [String::from("movq $42, %rbx")]
        );
    }
}
