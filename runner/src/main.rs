#![feature(iterator_try_collect)]

use std::fs::{read_to_string, write};

use errors::NilangError;
use nilang_generator::options::{AtAndTFlavour, SystemVAmd64Abi};

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let compiled = compile(&code);
    write("test.asm", compiled.as_ref()).unwrap();
}

fn compile(code: &str) -> Box<str> {
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

    let transformed = match nilang_transformer::transform(parsed) {
        Ok(transformed) => transformed,
        Err(err) => {
            panic!("{}", err);
        }
    };

    match nilang_generator::generate::<SystemVAmd64Abi, AtAndTFlavour>(transformed) {
        Ok(generated) => generated,
        Err(err) => {
            panic!("{}", err);
        }
    }
}
