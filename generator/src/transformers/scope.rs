use std::collections::HashMap;

use nilang_parser::nodes::Node;

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

    pub fn insert(&mut self, name: &str) -> i8 {
        let name = name.to_owned();
        if self.variables.contains_key(&name) {
            panic!("Variable {} already exists", name);
        }

        self.allocated += 1;

        let pointer_offset = -self.allocated * SINGLE_ALLOCATION_SIZE;
        self.variables.insert(name, pointer_offset);
        pointer_offset
    }
    pub fn insert_unnamed(&mut self) -> i8 {
        self.allocated += 1;
        -self.allocated * SINGLE_ALLOCATION_SIZE
    }
    pub fn get(&self, name: &str) -> i8 {
        *self
            .variables
            .get(name)
            .unwrap_or_else(|| panic!("Variable `{}` not declared", name))
    }
}

pub fn transform_scope(a: &Node, scope: &mut Scope) -> Vec<String> {
    if let Node::Scope(inner) = a {
        let mut scope = Scope::inherit(scope);

        let code = inner
            .iter()
            .map(|node| transform(node, &mut scope))
            .reduce(|a, b| [a, b].concat())
            .unwrap();

        [
            Vec::from([String::from("pushq %rbp"), String::from("movq %rsp, %rbp")]),
            code,
            Vec::from([String::from("leave")]),
        ]
        .concat()
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::scope::transform_scope;
    use nilang_parser::nodes::Node;

    #[test]
    fn scope_with_return() {
        let node = Node::Scope(vec![Node::Return(Box::new(Node::Number(42.)))]);
        let code = transform_scope(&node, &mut super::Scope::default());

        assert_eq!(
            &code,
            &[
                String::from("pushq %rbp"),
                String::from("movq %rsp, %rbp"),
                String::from("movq $42, %rbx"),
                String::from("leave")
            ]
        );
    }
}
