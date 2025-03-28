#![feature(iterator_try_collect)]

use std::fs::{read_to_string, write};

use errors::NilangError;
use nilang_generator::options::{AtAndTFlavour, SystemVAmd64Abi};
use nilang_transformer::{FunctionsRef, TypesRef};

fn main() {
    let code = read_to_string("test.ni").unwrap();

    let compiled = compile(&code);
    write("test.asm", compiled.as_ref()).unwrap();
}

fn compile(code: &str) -> Box<str> {
    let lexed = nilang_lexer::lex(code);

    let (functions, structures) = match nilang_parser::parse(lexed) {
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

    let context = create_transformer_context(&functions, &structures);

    let assembly = functions
        .iter()
        .map(|function| {
            (
                function.name.clone(),
                nilang_transformer::transform_function(&context, function),
            )
        })
        .map(|(name, result)| match result {
            Ok(asm) => (name, asm),
            Err(err) => {
                panic!("{}", err);
            }
        })
        .map(|(name, instructions)| {
            nilang_generator::generate_function::<SystemVAmd64Abi, AtAndTFlavour>(
                name,
                instructions,
            )
        })
        .map(|result| match result {
            Ok(asm) => asm,
            Err(err) => {
                panic!("{}", err);
            }
        });

    nilang_generator::generate_program::<AtAndTFlavour>(assembly)
}

fn create_transformer_context(
    functions: &[nilang_types::nodes::FunctionDeclaration],
    structures: &[nilang_types::nodes::StructureDeclaration],
) -> (FunctionsRef, TypesRef) {
    (functions.into(), structures.into())
}
