use std::collections::HashMap;

use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::to_assembly::ToAssembly;

pub trait Flavour<R>
where
    R: ToAssembly + Registers + Copy,
{
    fn generate(&mut self, instruction: Instruction) -> Result<Vec<Box<str>>, GeneratorErrors>;

    #[inline]
    fn get_return_register() -> impl ToAssembly {
        R::get_return_register()
    }
}

impl<R> Default for Inst<R>
where
    R: ToAssembly + Registers + Clone,
{
    fn default() -> Self {
        Self {
            free_registers: R::get_general_purpose_registers().to_vec(),
            reservations: HashMap::new(),
            allocated_stack: 0,
        }
    }
}

pub struct Inst<R>
where
    R: ToAssembly + Registers,
{
    free_registers: Vec<R>,
    reservations: HashMap<Box<str>, Location<R>>,
    allocated_stack: usize,
}

impl<R> Inst<R>
where
    R: ToAssembly + Registers + Copy,
{
    #[inline]
    pub fn reserve(&mut self, name: &str) -> impl ToAssembly {
        let location = match self.free_registers.pop() {
            Some(register) => Location::Register(register),
            None => {
                self.allocated_stack += 8;
                Location::Stack(self.allocated_stack)
            }
        };

        self.reservations.insert(name.into(), location);

        location
    }

    #[inline]
    pub fn get(&self, name: &str) -> Result<impl ToAssembly, GeneratorErrors> {
        match self.reservations.get(name) {
            Some(location) => Ok(*location),
            None => Err(GeneratorErrors::VariableNotDefined { name: name.into() }),
        }
    }
}

pub trait Instructions<S, R>
where
    S: StackManagement<RegisterSet = R>,
{
    fn generate(&mut self, instruction: Instruction) -> Box<[Box<str>]>;
}

pub trait StackManagement {
    type RegisterSet: ToAssembly + Registers;

    #[inline]
    fn get_return_register() -> impl ToAssembly {
        Self::RegisterSet::get_return_register()
    }

    #[inline]
    fn get_stack_pointer_register() -> impl ToAssembly {
        Self::RegisterSet::get_stack_pointer_register()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location<R>
where
    R: ToAssembly + Registers,
{
    Register(R),
    Stack(usize),
}

impl<R> ToAssembly for Location<R>
where
    R: ToAssembly + Registers,
{
    fn to_assembly(&self) -> Box<str> {
        match self {
            Location::Register(register) => register.to_assembly(),
            Location::Stack(offset) => unimplemented!(),
        }
    }
}

pub trait Registers: Sized {
    fn get_return_register() -> Self;
    fn get_stack_pointer_register() -> Self;

    fn get_general_purpose_registers() -> Box<[Self]>;
}
