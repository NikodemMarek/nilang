use nilang_types::nodes::Node;

use crate::{transformers::transform, utils::generate_function, TypesRef};

use super::{function_call::REGISTERS, scope::Scope};

pub fn transform_function_declaration(
    a: &Node,
    tr: &TypesRef,
    scope: &mut Scope,
) -> eyre::Result<Vec<String>> {
    if let Node::FunctionDeclaration {
        name,
        parameters,
        body,
        ..
    } = a
    {
        let mut code = Vec::new();

        let mut scope = Scope::inherit(scope);
        for (num, parameter) in parameters.iter().enumerate() {
            let offset = scope.insert(parameter.0, parameter.1)?;
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
                transform(body, tr, &mut scope)?,
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

    use crate::{
        transformers::{function_declaration::transform_function_declaration, scope::Scope},
        TypesRef,
    };

    #[test]
    fn test_function_declaration() {
        assert_eq!(
            transform_function_declaration(
                &(Node::FunctionDeclaration {
                    name: "main".into(),
                    parameters: [].into(),
                    return_type: "int".into(),
                    body: Box::new(Node::Scope(Vec::from([Node::Return(Box::new(
                        Node::Number(6.),
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
                String::from("    movq $6, %rbx"),
                String::from("    leave"),
                String::from("    ret"),
                String::new(),
            ]
        );
    }
}
