use std::fmt::Debug;

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

impl<R: Registers + Debug> Flavour for GnuFlavour<R> {
    type RegistersSet = R;

    #[inline]
    fn location(location: Location<Self::RegistersSet>) -> Box<str> {
        match location {
            Location::Register(register) => format!("%{}", register.to_assembly()).into(),
            Location::Stack(offset) => format!(
                "-{}({})",
                offset,
                Self::location(Self::base_pointer_register_location())
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
            Instruction::FunctionCall(name, arguments, return_temporary) => {
                let mut args = Vec::new();
                for arg_instructions in self.mm.put_arguments(&arguments).iter() {
                    args.extend(self.generate_instruction(arg_instructions.clone())?);
                }

                Ok([
                    vec![format!("call _{}", name).into()],
                    args,
                    vec![format!(
                        "movq {}, {}",
                        Self::location(Self::return_register_location()),
                        Self::location(self.mm.reserve(&return_temporary))
                    )
                    .into()],
                ]
                .concat())
            }
            Instruction::LoadArgument(arg, variable) => {
                let arg_loc = self.mm.get_argument(arg);
                let arg_var_loc = self.mm.reserve(&variable);
                Ok(vec![format!(
                    "movq {}, {}",
                    Self::location(arg_loc),
                    Self::location(arg_var_loc)
                )
                .into()])
            }
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
                                Self::location(Self::base_pointer_register_location())
                            )
                            .into(),
                            format!(
                                "movq {}, {}",
                                Self::location(Self::stack_pointer_register_location()),
                                Self::location(Self::base_pointer_register_location()),
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
        let start_fn = space_bottom(
            &[
                vec![".globl _start".into(), "_start:".into()],
                pad_lines(
                    &[
                        "call _main".into(),
                        // "movq $60, %rax".into(),
                        // "xorq %rdi, %rdi".into(),
                        // "syscall".into(),
                        "movq %rax, %rbx".into(),
                        "movq $1, %rax".into(),
                        "int $0x80".into(),
                        "ret".into(),
                    ],
                    4,
                ),
            ]
            .concat(),
        );

        format!(".text\n{}", &[start_fn, code.into()].concat().join("\n")).into()
    }
}
