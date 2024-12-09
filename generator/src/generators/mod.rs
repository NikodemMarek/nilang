use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::memory_manager::MemoryManager;

pub fn generate<I: Iterator<Item = Instruction>>(
    mm: &mut MemoryManager,
    instructions: &mut I,
) -> Result<Vec<Box<str>>, GeneratorErrors> {
    Ok(instructions
        .map(|instruction| match instruction {
            Instruction::LoadNumber(number, temporary) => {
                format!("movq ${}, {}", number, mm.reserve(&temporary)).into()
            }
            Instruction::ReturnNumber(number) => format!("movq ${}, %rbx", number).into(),
            _ => unimplemented!(),
        })
        .collect())
}
