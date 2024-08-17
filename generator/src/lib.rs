mod transformers;
mod utils;

use nilang_parser::{nodes::Node, parse};
use transformers::transform;
use utils::generate_function;

pub fn compile(input: &str) -> String {
    let program = parse(input);
    generate(program)
}

fn generate(program: Vec<Node>) -> String {
    let mut data = Vec::with_capacity(1024);
    let mut code = Vec::with_capacity(1024);

    for node in program {
        let (d, c) = transform(&node);
        data = [data, d].concat();
        code = [code, c].concat();
    }

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
