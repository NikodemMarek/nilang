#![feature(iterator_try_collect)]

use std::fs::{read_to_string, write};

use errors::{NilangError, TransformerErrors};
use nilang_generator::options::{AtAndTFlavour, SystemVAmd64Abi};
use nilang_transformer::{FunctionsRef, StructuresRef};

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

    let context = create_transformer_context(&functions, &structures).unwrap();

    let transformed = functions.iter().map(|function| {
        (
            function.name.clone(),
            nilang_transformer::transform_function(&context, function).map(|result| match result {
                Ok(instruction) => instruction,
                Err(err) => {
                    panic!("{}", err);
                }
            }),
        )
    });

    let generated = transformed.map(|(name, instructions)| {
        nilang_generator::generate_function::<SystemVAmd64Abi, AtAndTFlavour>(name, instructions)
            .map(|result| match result {
                Ok(instruction) => instruction,
                Err(err) => {
                    panic!("{}", err);
                }
            })
            .collect::<String>()
    });

    nilang_generator::generate_program::<AtAndTFlavour>()
        .chain(generated)
        .collect::<Vec<_>>()
        .join("\n")
        .into()
}

fn create_transformer_context(
    functions: &[nilang_types::nodes::FunctionDeclaration],
    structures: &[nilang_types::nodes::StructureDeclaration],
) -> Result<(FunctionsRef, StructuresRef), TransformerErrors> {
    structures.try_into().map(|s| (functions.into(), s))
}
