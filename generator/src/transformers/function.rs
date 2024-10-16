use nilang_parser::nodes::Node;

use crate::{transformers::transform, utils::generate_function};

use super::scope::Scope;

pub fn transform_function_declaration(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::FunctionDeclaration {
        name,
        parameters,
        body,
    } = a
    {
        let mut code = Vec::new();

        let mut scope = Scope::inherit(scope);
        for (index, parameter) in parameters.iter().enumerate() {
            // TODO: Take parameters from the stack
            let offset = scope.insert(parameter)?;
            code.push(format!("    movq -{}(%rsp), %rax", index * 8));
            code.push(format!("    movq %rax, {}(%rsp)", offset));
        }

        code.extend(generate_function(name, &transform(body, &mut scope)?));
        Ok(code)
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::{function_declaration::transform_function_declaration, scope::Scope};
    use nilang_parser::nodes::Node;

    #[test]
    fn function() {
        let node = Node::FunctionDeclaration {
            name: String::from("main"),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from([Node::Return(Box::new(
                Node::Number(6.),
            ))]))),
        };
        let code = transform_function_declaration(&node, &mut Scope::default());

        assert_eq!(
            code.unwrap(),
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    pushq %rbp"),
                String::from("    movq %rsp, %rbp"),
                String::from("    movq $6, %rbx"),
                String::from("    leave"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }
}
