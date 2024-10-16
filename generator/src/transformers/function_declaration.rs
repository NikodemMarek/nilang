use nilang_types::nodes::Node;

use crate::{transformers::transform, utils::generate_function};

use super::{function_call::REGISTERS, scope::Scope};

pub fn transform_function_declaration(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::FunctionDeclaration {
        name,
        parameters,
        body,
    } = a
    {
        let mut code = Vec::new();

        let mut scope = Scope::inherit(scope);
        for (num, parameter) in parameters.iter().enumerate() {
            let offset = scope.insert(parameter)?;
            if num < REGISTERS.len() {
                code.push(format!("movq %{}, {}(%rsp)", REGISTERS[num], offset));
            } else {
                // TODO: Take parameters from the stack
            }
        }

        Ok(generate_function(
            name,
            &[
                Vec::from([String::from("pushq %rbp"), String::from("movq %rsp, %rbp")]),
                code,
                transform(body, &mut scope)?,
                Vec::from([String::from("leave")]),
            ]
            .concat(),
        ))
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::Node;

    use crate::transformers::{function_declaration::transform_function_declaration, scope::Scope};

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
