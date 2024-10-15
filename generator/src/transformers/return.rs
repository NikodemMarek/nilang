use nilang_parser::nodes::Node;

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
            Node::VariableDeclaration { .. } | Node::Return(_) => {
                panic!("Unexpected node: {:?}", a)
            }
        }
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::r#return::transform_return;
    use nilang_parser::nodes::Node;

    #[test]
    fn return_number() {
        let node = Node::Return(Box::new(Node::Number(42.)));
        let code = transform_return(&node, &mut super::Scope::default());

        assert_eq!(code.unwrap(), Vec::from([String::from("movq $42, %rbx")]));
    }

    #[test]
    fn return_variable_reference() {
        let node = Node::Return(Box::new(Node::VariableReference(String::from("a"))));
        let mut scope = super::Scope::default();
        let _ = scope.insert("a");
        let code = transform_return(&node, &mut scope);

        assert_eq!(
            code.unwrap(),
            Vec::from([String::from("movq -8(%rbp), %rbx")])
        );
    }

    #[test]
    fn return_operation() {
        let node = Node::Return(Box::new(Node::Operation {
            operator: nilang_parser::nodes::Operator::Add,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        }));
        let code = transform_return(&node, &mut super::Scope::default());

        assert_eq!(code.unwrap(), Vec::from(["movq $1, %rbx", "add $2, %rbx"]));
    }
}
