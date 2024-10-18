#![feature(iterator_try_collect)]

use std::fs::{read_to_string, write};

use errors::{NilangError, ParserErrors};

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let compiled = compile(&code);
    write("test.asm", compiled).unwrap();
}

fn compile(code: &str) -> String {
    let mut lex = nilang_lexer::lex(code);
    let lexed = match lex.try_collect::<Vec<_>>() {
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

    let parsed = match nilang_parser::parse(&lexed) {
        Ok(parsed) => parsed,
        Err(err) => {
            let (start, end, message): ((usize, usize), (usize, usize), String) = err
                .root_cause()
                .downcast_ref::<ParserErrors>()
                .unwrap()
                .into();

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
