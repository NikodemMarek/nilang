use std::fs::{read_to_string, write};

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let hw = compile(&code);

    write("test.asm", hw).unwrap();
}

fn compile(input: &str) -> String {
    nilang_generator::generate(nilang_parser::parse(&nilang_lexer::lex(input)))
}
