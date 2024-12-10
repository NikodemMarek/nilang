use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    flavour::{Flavour, Location, MemoryManagement, Registers},
    to_assembly::ToAssembly,
};

pub struct GnuFlavour<R: Registers> {
    mm: MemoryManagement<R>,
}

impl<R: Registers> Default for GnuFlavour<R> {
    fn default() -> Self {
        Self {
            mm: Default::default(),
        }
    }
}

impl<R: Registers> Flavour for GnuFlavour<R> {
    type RegistersSet = R;

    fn location(location: Location<R>) -> Box<str> {
        match location {
            Location::Register(register) => register.to_assembly(),
            Location::Stack(offset) => format!(
                "-{}(%{})",
                offset,
                Self::get_stack_pointer_register().to_assembly()
            )
            .into(),
        }
    }

    fn generate(&mut self, instruction: Instruction) -> Result<Vec<Box<str>>, GeneratorErrors> {
        match instruction {
            Instruction::LoadNumber(number, temporary) => Ok(Vec::from([format!(
                "movq ${}, {}",
                number,
                GnuFlavour::location(self.mm.reserve(&temporary))
            )
            .into()])),
            Instruction::ReturnNumber(number) => Ok(Vec::from([
                format!(
                    "movq ${}, {}",
                    number,
                    Self::get_return_register().to_assembly()
                )
                .into(),
                "leave".into(),
            ])),
            Instruction::ReturnVariable(variable) => match self.mm.get(&variable) {
                Ok(location) => Ok(Vec::from([
                    format!(
                        "movq {}, {}",
                        Self::location(location),
                        Self::get_return_register().to_assembly()
                    )
                    .into(),
                    "leave".into(),
                ])),
                Err(e) => Err(e),
            },

            _ => unimplemented!(),
        }
    }
}
