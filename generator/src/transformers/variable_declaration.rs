use nilang_parser::nodes::Node;

use super::{function_call::transform_function_call, operator::transform_operation, scope::Scope};

pub fn transform_variable_declaration(node: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::VariableDeclaration { name, value } = node {
        match *value.to_owned() {
            Node::Number(num) => Ok(Vec::from([format!(
                "movq ${}, {}(%rbp)",
                num,
                scope.insert(name)?
            )])),
            Node::VariableReference(reference_name) => Ok(Vec::from([
                format!("movq {}(%rbp), %rax", scope.get(&reference_name)?),
                format!("movq %rax, {}(%rbp)", scope.insert(name)?),
            ])),
            node @ Node::Operation { .. } => Ok([
                transform_operation(&node, scope, "%rax")?,
                Vec::from([format!("movq %rax, {}(%rbp)", scope.insert(name)?)]),
            ]
            .concat()),
            node @ Node::FunctionCall { .. } => Ok([
                transform_function_call(&node, scope)?,
                Vec::from([format!("movq %rbx, {}(%rbp)", scope.insert(name)?)]),
            ]
            .concat()),
            _ => panic!("Unexpected node: {:?}", value),
        }
    } else {
        panic!("Unexpected node: {:?}", node)
    }
}

#[cfg(test)]
mod tests {
    use nilang_parser::nodes::Node;

    use super::transform_variable_declaration;

    #[test]
    fn variable_declaration_with_number() {
        let node = Node::VariableDeclaration {
            name: String::from("a"),
            value: Box::new(Node::Number(42.)),
        };
        let code = transform_variable_declaration(&node, &mut super::Scope::default());

        assert_eq!(
            code.unwrap(),
            Vec::from([String::from("movq $42, -8(%rbp)")])
        );
    }

    #[test]
    fn variable_declaration_with_reference() {
        let node = Node::VariableDeclaration {
            name: String::from("a"),
            value: Box::new(Node::VariableReference(String::from("b"))),
        };
        let mut scope = super::Scope::default();
        let _ = scope.insert("b");
        let code = transform_variable_declaration(&node, &mut scope);

        assert_eq!(
            code.unwrap(),
            Vec::from([
                String::from("movq -8(%rbp), %rax"),
                String::from("movq %rax, -16(%rbp)")
            ])
        );
    }

    #[test]
    fn variable_declaration_with_operation() {
        let node = Node::VariableDeclaration {
            name: String::from("a"),
            value: Box::new(Node::Operation {
                operator: nilang_parser::nodes::Operator::Add,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
        };
        let code = transform_variable_declaration(&node, &mut super::Scope::default());

        assert_eq!(
            code.unwrap(),
            Vec::from([
                String::from("movq $1, %rax"),
                String::from("add $2, %rax"),
                String::from("movq %rax, -8(%rbp)")
            ])
        );
    }
}
