use nilang_parser::nodes::Node;

use crate::{transformers::transform, utils::generate_function};

pub fn transform_function(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Function {
        name,
        parameters: _,
        body,
    } = a
    {
        let this = transform(body);
        (this.0, generate_function(name, &this.1))
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}
