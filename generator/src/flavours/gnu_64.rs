use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    flavour::{Flavour, Inst, Location, Registers, StackManagement},
    to_assembly::ToAssembly,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gnu64Registers {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
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
            Gnu64Registers::RAX => "rax",
            Gnu64Registers::RBX => "rbx",
            Gnu64Registers::RCX => "rcx",
            Gnu64Registers::RDX => "rdx",
            Gnu64Registers::RSI => "rsi",
            Gnu64Registers::RDI => "rdi",
            Gnu64Registers::RBP => "rbp",
            Gnu64Registers::RSP => "rsp",
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
    fn get_return_register() -> Self {
        Gnu64Registers::RSP
    }

    #[inline]
    fn get_stack_pointer_register() -> Self {
        Gnu64Registers::RBP
    }

    fn get_general_purpose_registers() -> Box<[Self]> {
        Box::new([
            Gnu64Registers::RAX,
            Gnu64Registers::RBX,
            Gnu64Registers::RCX,
            Gnu64Registers::RDX,
            Gnu64Registers::RSI,
            Gnu64Registers::RDI,
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

impl StackManagement for Location<Gnu64Registers> {
    type RegisterSet = Gnu64Registers;
}

// impl ToAssembly for Location<Gnu64Registers> {
//     fn to_assembly(&self) -> Box<str> {
//         match self {
//             Location::Register(register) => format!("%{}", register.to_assembly()).into(),
//             Location::Stack(offset) => format!(
//                 "-{}(%{})",
//                 offset,
//                 Gnu64Registers::get_stack_pointer_register().to_assembly()
//             )
//             .into(),
//         }
//     }
// }

#[derive(Default)]
pub struct GnuFlavour {
    mm: Inst<Gnu64Registers>,
}

impl Flavour<Gnu64Registers> for GnuFlavour {
    fn generate(&mut self, instruction: Instruction) -> Result<Vec<Box<str>>, GeneratorErrors> {
        match instruction {
            Instruction::LoadNumber(number, temporary) => Ok(Vec::from([format!(
                "movq ${}, {}",
                number,
                self.mm.reserve(&temporary).to_assembly()
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
                        location.to_assembly(),
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

impl Inst<Gnu64Registers> {}
