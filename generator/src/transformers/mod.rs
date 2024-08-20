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

#[cfg(test)]
mod tests {
    use crate::transformers::transform;
    use nilang_parser::nodes::{Node, Operator};

    #[test]
    fn transform_main_function() {
        let node = Node::Function {
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
        let (data, code) = transform(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    push $1"),
                String::from("    push $2"),
                String::from("    pop %rbx"),
                String::from("    pop %rax"),
                String::from("    add %rbx, %rax"),
                String::from("    push %rax"),
                String::from("    pop %rax"),
                String::from("    movl %eax, %ebx"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }
}
