#![feature(iterator_try_collect)]

use std::fs::{read_to_string, write};

use errors::NilangError;

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let compiled = compile(&code);
    write("test.asm", compiled).unwrap();
}

fn compile(code: &str) -> String {
    let lexed = nilang_lexer::lex(code);

    let parsed = match nilang_parser::parse(lexed) {
        Ok(parsed) => parsed,
        Err(err) => {
            let (start, end, message): ((usize, usize), (usize, usize), String) = (&err).into();

            panic!(
                "{}",
                NilangError {
                    code: code.to_owned(),
                    start,
                    end,
                    message,
                }
            );
        }
    };

    match nilang_generator::generate(parsed) {
        Ok(generated) => generated,
        Err(err) => {
            panic!("{}", err);
        }
    }
}
