#![feature(iterator_try_collect)]

use std::{
    cell::RefCell,
    fs::{read_to_string, write},
};

use errors::TransformerErrors;
use nilang_generator::options::{AtAndTFlavour, SystemVAmd64Abi, X86Registers};
use nilang_transformer::{FunctionsRef, StructuresRef};
use nilang_types::Localizable;

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
            panic!("{}", err.format_error(code));
        }
    };

    let context = create_transformer_context(&functions, &structures).unwrap();

    let data = RefCell::new(Vec::new());
    let transformed = functions.iter().map(|function| {
        let (function_instructions, mut new_data) =
            nilang_transformer::transform_function(&context, function);
        data.borrow_mut().append(&mut new_data);
        (
            function.name.clone(),
            function_instructions.map(|result| match result {
                Ok(instruction) => instruction,
                Err(err) => {
                    panic!("{}", err);
                }
            }),
        )
    });

    let generated = transformed.map(|(name, instructions)| {
        nilang_generator::generate_function::<X86Registers, SystemVAmd64Abi, AtAndTFlavour>(
            (*name).clone(),
            &data
                .borrow()
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>(),
            instructions,
        )
        .map(|result| match result {
            Ok(instruction) => instruction,
            Err(err) => {
                panic!("{}", err);
            }
        })
        .collect::<String>()
    });

    let code = nilang_generator::generate_program::<AtAndTFlavour>()
        .chain(generated)
        .collect::<Vec<_>>()
        .join("\n");

    let code = ".data\n".to_owned()
        + &nilang_generator::generate_data::<AtAndTFlavour>(&data.borrow()).collect::<String>()
        + &code;
    code.into()
}

fn create_transformer_context(
    functions: &[Localizable<nilang_types::nodes::FunctionDeclaration>],
    structures: &[Localizable<nilang_types::nodes::StructureDeclaration>],
) -> Result<(FunctionsRef, StructuresRef), TransformerErrors> {
    TryInto::try_into(structures).map(|s| (functions.into(), s))
}
