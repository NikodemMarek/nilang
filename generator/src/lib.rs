#![feature(box_patterns)]

mod transformers;
mod utils;

use errors::GeneratorErrors;
use nilang_types::nodes::Node;
use transformers::{scope::Scope, transform};
use utils::generate_function;

pub fn generate(program: Node) -> eyre::Result<String> {
    if let Node::Program(program) = program {
        let mut scope = Scope::default();

        let mut code = Vec::with_capacity(4096);
        for node in program {
            code.append(&mut transform(&node, &mut scope)?);
        }

        Ok(generate_program(&[], &code))
    } else {
        Err(GeneratorErrors::InvalidNode { node: program })?
    }
}

fn generate_program(data: &[String], code: &[String]) -> String {
    let start_fn = generate_function(
        "start",
        &[
            String::from("call _main"),
            String::from("movl $1, %eax"),
            // String::from("movl $0, %ebx"),
            String::from("int $0x80"),
        ],
    );

    format!(
        ".data\n{}\n.text\n{}",
        &data.join("\n"),
        &[start_fn, code.to_vec()].concat().join("\n")
    )
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::{generate, generate_program};

    #[test]
    fn test_generate() {
        let node = Node::Program(Vec::from([Node::FunctionDeclaration {
            name: "main".to_string(),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                },
            ))]))),
        }]));
        let output = generate(node);

        assert_eq!(
            output.unwrap(),
            ".data\n\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    int $0x80\n    ret\n\n.globl _main\n_main:\n    pushq %rbp\n    movq %rsp, %rbp\n    movq $1, %rbx\n    add $2, %rbx\n    leave\n    ret\n"
        )
    }

    #[test]
    fn test_generate_program() {
        assert_eq!(
            generate_program(
                &Vec::from([String::from("data")]),
                &Vec::from([String::from("code")]),
            ),
            ".data\ndata\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    int $0x80\n    ret\n\ncode"
        );
    }
}
