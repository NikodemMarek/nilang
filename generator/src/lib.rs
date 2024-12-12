#![feature(box_patterns)]

mod flavour;
mod flavours;
mod generator;
mod to_assembly;
mod utils;

use std::collections::HashMap;

use flavour::Flavour;
use flavours::{gnu_flavour::GnuFlavour, x86_registers::Gnu64Registers};
use nilang_types::instructions::Instruction;

pub fn generate(functions: HashMap<Box<str>, Vec<Instruction>>) -> eyre::Result<Box<str>> {
    let mut flavour = GnuFlavour::<Gnu64Registers>::default();
    let mut code = Vec::new();

    for (name, instructions) in functions.into_iter() {
        let mut function = GnuFlavour::<Gnu64Registers>::generate_function(
            &name,
            &generator::generate(&mut flavour, &mut instructions.into_iter())?,
        );

        code.append(&mut function);
    }

    Ok(GnuFlavour::<Gnu64Registers>::generate_program(&code))
}
