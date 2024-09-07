pub mod function;
pub mod operator;
pub mod r#return;
pub mod scope;
pub mod variable_declaration;

use nilang_parser::nodes::Node;
use scope::Scope;
use variable_declaration::transform_variable_declaration;

use crate::transformers::{
    function::transform_function, operator::transform_operation, r#return::transform_return,
    scope::transform_scope,
};

pub fn transform(node: &Node, scope: &mut Scope) -> (Vec<String>, Vec<String>) {
    match node {
        Node::Return(_) => transform_return(node, scope),
        Node::FunctionDeclaration { .. } => transform_function(node, scope),
        Node::Scope(_) => transform_scope(node, scope),
        Node::Operation { .. } => transform_operation(node, scope, "%rax"),
        Node::VariableDeclaration { .. } => transform_variable_declaration(node, scope),
        node @ Node::Number(_) | node @ Node::VariableReference(_) => {
            panic!("Unexpected node: {:?}", node)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::transform;
    use nilang_parser::nodes::{Node, Operator};

    #[test]
    fn transform_main_function() {
        let node = Node::FunctionDeclaration {
            name: "main".to_string(),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                },
            ))]))),
        };
        let (data, code) = transform(&node, &mut super::Scope::default());

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    pushq %rbp"),
                String::from("    movq %rsp, %rbp"),
                String::from("    movq $1, %rbx"),
                String::from("    add $2, %rbx"),
                String::from("    leave"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }
}
