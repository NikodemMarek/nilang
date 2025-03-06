use std::{collections::HashMap, fmt::Debug};

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
    fn stack_pointer_register_location() -> Location<Self::RegistersSet> {
        Location::Register(Self::RegistersSet::stack_pointer())
    }

    #[inline]
    fn base_pointer_register_location() -> Location<Self::RegistersSet> {
        Location::Register(Self::RegistersSet::base_pointer())
    }

    #[inline]
    fn return_register_location() -> Location<Self::RegistersSet> {
        Location::Register(Self::RegistersSet::return_register())
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

#[derive(Debug)]
pub struct MemoryManagement<R: Registers> {
    free_registers: Vec<R>,
    reservations: HashMap<Box<str>, Location<R>>,
    allocated_stack: usize,
}

impl<R: Registers + Debug> MemoryManagement<R> {
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

    #[inline]
    fn check_reservation(&self, loc: &Location<R>) -> Option<Box<str>> {
        self.reservations.iter().find_map(|(name, location)| {
            if location == loc {
                Some(name.to_owned())
            } else {
                None
            }
        })
    }

    pub fn put_arguments(&mut self, arguments: &[Box<str>]) -> Vec<Instruction> {
        let argument_locations = arguments
            .iter()
            .enumerate()
            .map(|(i, argument)| {
                (
                    argument.clone(),
                    if i >= R::argument_registers().len() {
                        Location::Stack((i - R::argument_registers().len()) * R::alignment())
                    } else {
                        Location::Register(R::argument_registers()[i])
                    },
                )
            })
            .collect::<Vec<(Box<str>, Location<R>)>>();

        let conflicting_reservations: Vec<_> = argument_locations
            .iter()
            .filter_map(|(argument, location)| {
                self.check_reservation(location)
                    .map(|name| (argument, name, location))
            })
            .collect();

        for (argument, location) in argument_locations.iter() {
            self.reservations.insert(argument.clone(), *location);
        }

        conflicting_reservations
            .iter()
            .map(|(argument, name, location)| {
                self.reservations.insert(name.clone(), **location);
                self.reserve(name);
                Instruction::Copy((*argument).clone(), name.clone())
            })
            .collect()
    }

    pub fn get_argument(&self, arg: usize) -> Location<R> {
        if arg >= R::argument_registers().len() {
            Location::Stack((arg - R::argument_registers().len()) * R::alignment())
        } else {
            Location::Register(R::argument_registers()[arg])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location<R: Registers> {
    Register(R),
    Stack(usize),
}

pub trait Registers: Sized + Copy + Clone + ToAssembly + PartialEq + Eq {
    fn alignment() -> usize;

    fn stack_pointer() -> Self;
    fn base_pointer() -> Self;
    fn return_register() -> Self;

    fn general_purpose_registers() -> Box<[Self]>;
    fn argument_registers() -> Box<[Self]>;
}
