use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    flavour::{Flavour, Location, MemoryManagement, Registers},
    to_assembly::ToAssembly,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gnu64Registers {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl ToAssembly for Gnu64Registers {
    fn to_assembly(&self) -> Box<str> {
        match self {
            Gnu64Registers::Rax => "rax",
            Gnu64Registers::Rbx => "rbx",
            Gnu64Registers::Rcx => "rcx",
            Gnu64Registers::Rdx => "rdx",
            Gnu64Registers::Rsi => "rsi",
            Gnu64Registers::Rdi => "rdi",
            Gnu64Registers::Rbp => "rbp",
            Gnu64Registers::Rsp => "rsp",
            Gnu64Registers::R8 => "r8",
            Gnu64Registers::R9 => "r9",
            Gnu64Registers::R10 => "r10",
            Gnu64Registers::R11 => "r11",
            Gnu64Registers::R12 => "r12",
            Gnu64Registers::R13 => "r13",
            Gnu64Registers::R14 => "r14",
            Gnu64Registers::R15 => "r15",
        }
        .into()
    }
}

impl Registers for Gnu64Registers {
    #[inline]
    fn alignment() -> usize {
        8
    }

    #[inline]
    fn return_register() -> Self {
        Gnu64Registers::Rsp
    }

    #[inline]
    fn stack_pointer_register() -> Self {
        Gnu64Registers::Rbp
    }

    fn general_purpose_registers() -> Box<[Self]> {
        Box::new([
            Gnu64Registers::Rax,
            Gnu64Registers::Rbx,
            Gnu64Registers::Rcx,
            Gnu64Registers::Rdx,
            Gnu64Registers::Rsi,
            Gnu64Registers::Rdi,
            Gnu64Registers::R8,
            Gnu64Registers::R9,
            Gnu64Registers::R10,
            Gnu64Registers::R11,
            Gnu64Registers::R12,
            Gnu64Registers::R13,
            Gnu64Registers::R14,
            Gnu64Registers::R15,
        ])
    }
}

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

impl MemoryManagement<Gnu64Registers> {}
