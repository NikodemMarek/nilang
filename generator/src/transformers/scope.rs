use nilang_parser::nodes::Node;

use crate::transformers::transform;

pub fn transform_scope(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Scope(inner) = a {
        inner
            .iter()
            .map(|node| {
                let this = transform(node);
                (this.0, this.1)
            })
            .reduce(|a, b| ([a.0, b.0].concat(), [a.1, b.1].concat()))
            .unwrap()
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn scope_with_return() {
        use crate::transformers::scope::transform_scope;
        use nilang_parser::nodes::Node;
        let node = Node::Scope(vec![Node::Return(Box::new(Node::Number(42.)))]);
        let (data, code) = transform_scope(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $42"),
                String::from("pop %rax"),
                String::from("movl %eax, %ebx")
            ])
        );
    }
}
