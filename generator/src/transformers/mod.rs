pub mod function_call;
pub mod function_declaration;
pub mod operator;
pub mod r#return;
pub mod scope;
pub mod variable_declaration;

use nilang_types::nodes::Node;
use scope::Scope;
use variable_declaration::transform_variable_declaration;

use crate::{
    transformers::{
        function_call::transform_function_call,
        function_declaration::transform_function_declaration, operator::transform_operation,
        r#return::transform_return, scope::transform_scope,
    },
    TypesRef,
};

pub fn transform(node: &Node, tr: &TypesRef, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    match node {
        Node::Return(_) => transform_return(node, scope),
        Node::FunctionDeclaration { .. } => transform_function_declaration(node, tr, scope),
        Node::Scope(_) => transform_scope(node, tr, scope),
        Node::Operation { .. } => transform_operation(node, scope, "%rax"),
        Node::VariableDeclaration { .. } => transform_variable_declaration(node, tr, scope),
        Node::FunctionCall { .. } => transform_function_call(node, scope),
        Node::Structure { .. } => todo!(),
        Node::Object { .. } => todo!(),
        node @ Node::Number(_)
        | node @ Node::VariableReference(_)
        | node @ Node::FieldAccess { .. } => {
            panic!("Unexpected node: {:?}", node)
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::{
        transformers::{scope::Scope, transform},
        TypesRef,
    };

    #[test]
    fn test_transform() {
        assert_eq!(
            transform(
                &(Node::FunctionDeclaration {
                    name: "main".into(),
                    parameters: [].into(),
                    return_type: "int".into(),
                    body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                        Node::Operation {
                            operator: Operator::Add,
                            a: Box::new(Node::Number(1.)),
                            b: Box::new(Node::Number(2.)),
                        },
                    ))]))),
                }),
                &TypesRef::default(),
                &mut Scope::default(),
            )
            .unwrap(),
            [
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    pushq %rbp"),
                String::from("    movq %rsp, %rbp"),
                String::from("    movq $1, %rbx"),
                String::from("    add $2, %rbx"),
                String::from("    leave"),
                String::from("    ret"),
                String::new(),
            ]
        );
    }
}
