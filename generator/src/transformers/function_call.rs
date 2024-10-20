use nilang_types::nodes::Node;

use super::{operator::transform_operation, scope::Scope};

pub const REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn transform_function_call(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::FunctionCall { name, arguments } = a {
        let mut code = Vec::new();

        for (num, argument) in arguments.iter().enumerate() {
            if num < REGISTERS.len() {
                match argument {
                    Node::Number(number) => {
                        code.push(format!("movq ${}, %{}", number, REGISTERS[num]));
                    }
                    Node::VariableReference(name) => {
                        let offset = scope.get(name).unwrap();
                        code.push(format!("movq {}(%rbp), %{}", offset, REGISTERS[num]));
                    }
                    node @ Node::Operation { .. } => {
                        code.extend(transform_operation(node, scope, "%rax")?);
                        code.push(format!("movq %rax, %{}", REGISTERS[num]));
                    }
                    node @ Node::FunctionCall { .. } => {
                        code.extend(transform_function_call(node, scope)?);
                        code.push(format!("movq %rbx, %{}", REGISTERS[num]));
                    }
                    Node::Scope(_) => todo!(),
                    _ => {}
                }
            } else {
                // TODO: Push to the stack
            }
        }

        code.push(format!("call _{}", name));

        Ok(code)
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::transformers::{function_call::transform_function_call, scope::Scope};

    #[test]
    fn test_function_call() {
        assert_eq!(
            transform_function_call(
                &(Node::FunctionCall {
                    name: String::from("main"),
                    arguments: Vec::from([
                        Node::Number(1.),
                        Node::Number(2.),
                        Node::Operation {
                            operator: Operator::Add,
                            a: Box::new(Node::Number(3.)),
                            b: Box::new(Node::Number(4.)),
                        },
                    ]),
                }),
                &mut Scope::default(),
            )
            .unwrap(),
            [
                String::from("movq $1, %rdi"),
                String::from("movq $2, %rsi"),
                String::from("movq $3, %rax"),
                String::from("add $4, %rax"),
                String::from("movq %rax, %rdx"),
                String::from("call _main"),
            ]
        );
    }
}
