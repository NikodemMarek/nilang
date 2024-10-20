use nilang_types::nodes::Node;

use super::{function_call::transform_function_call, operator::transform_operation, scope::Scope};

pub fn transform_return(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::Return(inner) = a {
        match *inner.clone() {
            Node::Number(value) => Ok(Vec::from([format!("movq ${}, %rbx", value)])),
            node @ Node::Operation { .. } => transform_operation(&node, scope, "%rbx"),
            Node::VariableReference(name) => Ok(Vec::from([format!(
                "movq {}(%rbp), %rbx",
                scope.get(&name)?
            )])),
            Node::FunctionDeclaration { .. } | Node::Scope(_) => todo!(),
            node @ Node::FunctionCall { .. } => transform_function_call(&node, scope),
            Node::VariableDeclaration { .. } | Node::Return(_) | Node::Program(_) => {
                panic!("Unexpected node: {:?}", a)
            }
        }
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::transformers::r#return::transform_return;

    #[test]
    fn return_number() {
        assert_eq!(
            transform_return(
                &Node::Return(Box::new(Node::Number(42.))),
                &mut super::Scope::default(),
            )
            .unwrap(),
            [String::from("movq $42, %rbx")]
        );
    }

    #[test]
    fn return_variable_reference() {
        let mut scope = super::Scope::default();
        let _ = scope.insert("a");

        assert_eq!(
            transform_return(
                &Node::Return(Box::new(Node::VariableReference(String::from("a")))),
                &mut scope,
            )
            .unwrap(),
            [String::from("movq -8(%rbp), %rbx")]
        );
    }

    #[test]
    fn return_operation() {
        assert_eq!(
            transform_return(
                &Node::Return(Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                })),
                &mut super::Scope::default(),
            )
            .unwrap(),
            ["movq $1, %rbx", "add $2, %rbx"]
        );
    }
}
