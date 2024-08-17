pub mod function;
pub mod operator;
pub mod r#return;
pub mod scope;

use nilang_parser::nodes::Node;

use crate::transformers::{
    function::transform_function, operator::transform_operation, r#return::transform_return,
    scope::transform_scope,
};

pub fn transform(node: &Node) -> (Vec<String>, Vec<String>) {
    match node {
        Node::Return(_) => transform_return(node),
        Node::Function { .. } => transform_function(node),
        Node::Scope(_) => transform_scope(node),
        Node::Operation { .. } => transform_operation(node),
        Node::Number(n) => (Vec::new(), Vec::from([format!("push ${}", n)])),
    }
}
