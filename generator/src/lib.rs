mod transformers;
mod utils;

use nilang_parser::nodes::Node;
use transformers::transform;
use utils::generate_function;

pub fn generate<I: IntoIterator<Item = Node>>(program: I) -> String {
    let (data, code) = program.into_iter().fold(
        (Vec::with_capacity(256), Vec::with_capacity(4096)),
        |(data, code), node| {
            let (d, c) = transform(&node);
            ([data, d].concat(), [code, c].concat())
        },
    );

    generate_program(&data, &code)
}

fn generate_program(data: &[String], code: &[String]) -> String {
    let start_fn = generate_function(
        "start",
        &[
            String::from("call _main"),
            String::from("movl $1, %eax"),
            String::from("movl $0, %ebx"),
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
    #[test]
    fn generate() {
        use nilang_parser::nodes::{Node, Operator};
        let node = Node::Function {
            name: "main".to_string(),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                },
            ))]))),
        };
        let output = super::generate(std::iter::once(node));

        assert_eq!(
            output,
            ".data\n\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    movl $0, %ebx\n    int $0x80\n    ret\n\n.globl _main\n_main:\n    push $1\n    push $2\n    pop %rbx\n    pop %rax\n    add %rbx, %rax\n    push %rax\n    pop %rax\n    movl %eax, %ebx\n    ret\n"
        )
    }

    #[test]
    fn generate_program() {
        let data = Vec::from([String::from("data")]);
        let code = Vec::from([String::from("code")]);
        let output = super::generate_program(&data, &code);

        assert_eq!(output, ".data\ndata\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    movl $0, %ebx\n    int $0x80\n    ret\n\ncode")
    }
}
