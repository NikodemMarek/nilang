use std::fs::{read_to_string, write};

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let hw = compile(&code);

    write("test.asm", hw).unwrap();
}

fn compile(input: &str) -> String {
    let lexed = nilang_lexer::lex(input);
    let parsed = nilang_parser::parse(&lexed);
    dbg!(&parsed);
    nilang_generator::generate(parsed)
}
