use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    flavour::{Flavour, Location, MemoryManagement, Registers},
    utils::{pad_lines, space_bottom},
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

    #[inline]
    fn location(location: Location<Self::RegistersSet>) -> Box<str> {
        match location {
            Location::Register(register) => format!("%{}", register.to_assembly()).into(),
            Location::Stack(offset) => format!(
                "-{}({})",
                offset,
                Self::location(Self::stack_pointer_register_location())
            )
            .into(),
        }
    }

    fn generate_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<Vec<Box<str>>, GeneratorErrors> {
        match instruction {
            Instruction::LoadNumber(number, temporary) => Ok(Vec::from([format!(
                "movq ${}, {}",
                number,
                Self::location(self.mm.reserve(&temporary))
            )
            .into()])),
            Instruction::ReturnNumber(number) => Ok(Vec::from([
                format!(
                    "movq ${}, {}",
                    number,
                    Self::location(Self::return_register_location())
                )
                .into(),
                "leave".into(),
            ])),
            Instruction::ReturnVariable(variable) => match self.mm.get(&variable) {
                Ok(location) => Ok(Vec::from([
                    format!(
                        "movq {}, {}",
                        Self::location(location),
                        Self::location(Self::return_register_location())
                    )
                    .into(),
                    "leave".into(),
                ])),
                Err(e) => Err(e),
            },

            _ => unimplemented!(),
        }
    }

    fn generate_function(name: &str, code: &[Box<str>]) -> Vec<Box<str>> {
        space_bottom(
            &[
                vec![format!(".globl _{name}").into(), format!("_{name}:").into()],
                pad_lines(
                    [
                        vec![
                            format!(
                                "pushq {}",
                                Self::location(Self::stack_pointer_register_location())
                            )
                            .into(),
                            format!(
                                "movq {}, {}",
                                Self::location(Self::stack_pointer_register_location()),
                                Self::location(Self::return_register_location())
                            )
                            .into(),
                        ],
                        code.into(),
                        vec!["ret".into()],
                    ]
                    .concat()
                    .iter(),
                    4,
                ),
            ]
            .concat(),
        )
    }

    fn generate_program(code: &[Box<str>]) -> Box<str> {
        let start_fn = Self::generate_function(
            "start",
            &[
                "call _main".into(),
                "movl $1, %eax".into(),
                // String::from("movl $0, %ebx"),
                "int $0x80".into(),
            ],
        );

        format!(".text\n{}", &[start_fn, code.into()].concat().join("\n")).into()
    }
}
