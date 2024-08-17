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
