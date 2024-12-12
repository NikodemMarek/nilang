use std::collections::HashMap;

use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::to_assembly::ToAssembly;

pub trait Flavour {
    type RegistersSet: Registers;

    fn location(location: Location<Self::RegistersSet>) -> Box<str>;

    fn generate_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<Vec<Box<str>>, GeneratorErrors>;

    fn generate_function(name: &str, code: &[Box<str>]) -> Vec<Box<str>>;
    fn generate_program(code: &[Box<str>]) -> Box<str>;

    #[inline]
    fn return_register_location() -> Location<Self::RegistersSet> {
        Location::Register(Self::RegistersSet::return_register())
    }

    #[inline]
    fn stack_pointer_register_location() -> Location<Self::RegistersSet> {
        Location::Register(Self::RegistersSet::stack_pointer_register())
    }
}

impl<R: Registers> Default for MemoryManagement<R> {
    fn default() -> Self {
        Self {
            free_registers: R::general_purpose_registers().to_vec(),
            reservations: HashMap::new(),
            allocated_stack: 0,
        }
    }
}

pub struct MemoryManagement<R: Registers> {
    free_registers: Vec<R>,
    reservations: HashMap<Box<str>, Location<R>>,
    allocated_stack: usize,
}

impl<R: Registers> MemoryManagement<R> {
    #[inline]
    pub fn reserve(&mut self, name: &str) -> Location<R> {
        let location = match self.free_registers.pop() {
            Some(register) => Location::Register(register),
            None => {
                self.allocated_stack += R::alignment();
                Location::Stack(self.allocated_stack)
            }
        };

        self.reservations.insert(name.into(), location);

        location
    }

    #[inline]
    pub fn get(&self, name: &str) -> Result<Location<R>, GeneratorErrors> {
        match self.reservations.get(name) {
            Some(location) => Ok(*location),
            None => Err(GeneratorErrors::VariableNotDefined { name: name.into() }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location<R: Registers> {
    Register(R),
    Stack(usize),
}

pub trait Registers: Sized + Copy + Clone + ToAssembly {
    fn alignment() -> usize;

    fn return_register() -> Self;
    fn stack_pointer_register() -> Self;

    fn general_purpose_registers() -> Box<[Self]>;
}
