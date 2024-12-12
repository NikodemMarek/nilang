use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::flavour::Flavour;

pub fn generate<I, F>(
    flavour: &mut F,
    instructions: &mut I,
) -> Result<Box<[Box<str>]>, GeneratorErrors>
where
    I: Iterator<Item = Instruction>,
    F: Flavour,
{
    instructions
        .map(|instruction| flavour.generate_instruction(instruction))
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(er) => Vec::from([Err(er)]),
        })
        .collect()
}
