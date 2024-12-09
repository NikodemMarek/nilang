use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    flavour::{Flavour, Registers},
    to_assembly::ToAssembly,
};

pub fn generate<R, I, F>(
    flavour: &mut F,
    instructions: &mut I,
) -> Result<Box<[Box<str>]>, GeneratorErrors>
where
    I: Iterator<Item = Instruction>,
    R: ToAssembly + Registers + Copy,
    F: Flavour<R>,
{
    instructions
        .map(|instruction| flavour.generate(instruction))
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(er) => Vec::from([Err(er)]),
        })
        .collect()
}
