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
