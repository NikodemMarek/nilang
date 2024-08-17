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
