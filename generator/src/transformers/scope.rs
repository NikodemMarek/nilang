use std::collections::HashMap;

use errors::GeneratorErrors;
use nilang_types::nodes::Node;

use crate::transformers::transform;

#[derive(Default)]
pub struct Scope {
    variables: HashMap<String, i8>,
    allocated: i8,
}

static SINGLE_ALLOCATION_SIZE: i8 = 8;

impl Scope {
    pub fn inherit(scope: &Scope) -> Self {
        Scope {
            variables: scope.variables.clone(),
            allocated: scope.allocated,
        }
    }

    pub fn insert(&mut self, name: &str) -> eyre::Result<i8> {
        let name = name.to_owned();
        if self.variables.contains_key(&name) {
            Err(GeneratorErrors::VariableAlreadyExists { name: name.clone() })?
        }

        self.allocated += 1;

        let pointer_offset = -self.allocated * SINGLE_ALLOCATION_SIZE;
        self.variables.insert(name, pointer_offset);
        Ok(pointer_offset)
    }
    pub fn insert_unnamed(&mut self) -> eyre::Result<i8> {
        self.allocated += 1;
        Ok(-self.allocated * SINGLE_ALLOCATION_SIZE)
    }

    pub fn get(&self, name: &str) -> eyre::Result<i8> {
        let offset = match self.variables.get(name) {
            Some(offset) => *offset,
            None => Err(GeneratorErrors::VariableDoesNotExist {
                name: String::from(name),
            })?,
        };

        Ok(offset)
    }
}

pub fn transform_scope(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::Scope(inner) = a {
        let mut scope = Scope::inherit(scope);

        let mut code = Vec::with_capacity(4096);
        for node in inner {
            code.append(&mut transform(node, &mut scope)?);
        }

        Ok(code)
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::Node;

    use crate::transformers::scope::transform_scope;

    #[test]
    fn scope_with_return() {
        assert_eq!(
            transform_scope(
                &Node::Scope(vec![Node::Return(Box::new(Node::Number(42.)))]),
                &mut super::Scope::default(),
            )
            .unwrap(),
            [String::from("movq $42, %rbx")]
        );
    }
}
