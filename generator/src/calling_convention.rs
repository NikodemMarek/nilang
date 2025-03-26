use errors::GeneratorErrors;

use crate::{
    assembly_flavour::{AssemblyInstruction, AssemblyInstructionParameter, FullInstruction},
    memory_manager::{Location, MemoryManager},
    registers::Registers,
};

pub trait CallingConvention {
    fn generate_function_call(
        mm: &mut MemoryManager,
        name: &str,
        args: &[Box<str>],
        return_temporary: Box<str>,
    ) -> Result<Vec<FullInstruction>, GeneratorErrors>;

    fn return_location() -> Location;
    fn nth_argument_location(n: usize) -> Location;
}

pub struct SystemVAmd64Abi;

impl SystemVAmd64Abi {
    fn arguments_locations(arguments: &[Box<str>]) -> Vec<Location> {
        let available_registers = [
            Registers::Rdi,
            Registers::Rsi,
            Registers::Rdx,
            Registers::Rcx,
            Registers::R8,
            Registers::R9,
        ];
        if arguments.len() <= available_registers.len() {
            available_registers[..arguments.len()]
                .iter()
                .map(|r| Location::Register(r.clone()))
                .collect()
        } else {
            [
                available_registers
                    .iter()
                    .map(|r| Location::Register(r.clone()))
                    .collect::<Vec<_>>(),
                (0..arguments.len() - available_registers.len())
                    .map(|i| Location::Stack(i * 8))
                    .collect::<Vec<_>>(),
            ]
            .concat()
        }
    }

    fn relocations(
        mm: &mut MemoryManager,
        arguments_locations: &[Location],
    ) -> Vec<FullInstruction> {
        arguments_locations
            .iter()
            .filter_map(|loc| {
                if !mm.is_taken(loc) {
                    return None;
                }

                let var = mm.get_name(loc).unwrap().clone();
                mm.free(&var);
                let new_loc = mm.reserve(&var);
                Some((
                    AssemblyInstruction::Move,
                    vec![new_loc.into(), loc.into()],
                    format!("Move `{var}` to a free location").into(),
                ))
            })
            .collect()
    }
}

impl CallingConvention for SystemVAmd64Abi {
    fn generate_function_call(
        mm: &mut MemoryManager,
        name: &str,
        args: &[Box<str>],
        return_temporary: Box<str>,
    ) -> Result<Vec<FullInstruction>, GeneratorErrors> {
        let arguments_locations = SystemVAmd64Abi::arguments_locations(args);

        let how_many_are_taken = arguments_locations
            .iter()
            .map(|arg| mm.is_taken(arg))
            .filter(|is_taken| *is_taken)
            .count();
        mm.add_n_next_locations(how_many_are_taken);

        let relocations = Self::relocations(mm, arguments_locations.as_slice());

        let arguments_allocations = arguments_locations
            .iter()
            .zip(args.iter())
            .enumerate()
            .map(|(i, (loc, arg))| {
                let arg_loc = mm.get_location(arg).unwrap();
                (
                    AssemblyInstruction::Move,
                    vec![loc.into(), arg_loc.into()],
                    format!("Load `{arg}` as argument {i}").into(),
                )
            })
            .collect();

        let stack_alignment = [];

        let function_call = [(
            AssemblyInstruction::Call,
            vec![AssemblyInstructionParameter::Function(name.into())],
            format!("Call function `{name}`").into(),
        )];

        let stack_cleanup = [];

        let return_register = mm.reserve(&return_temporary);
        let move_result = [(
            AssemblyInstruction::Move,
            vec![
                return_register.into(),
                SystemVAmd64Abi::return_location().into(),
            ],
            format!("Move result of `{name}` to return register").into(),
        )];

        Ok([
            relocations,
            arguments_allocations,
            stack_alignment.into(),
            function_call.into(),
            stack_cleanup.into(),
            move_result.into(),
        ]
        .concat())
    }

    fn return_location() -> Location {
        Location::Register(Registers::Rax)
    }

    fn nth_argument_location(n: usize) -> Location {
        match n {
            0 => Location::Register(Registers::Rdi),
            1 => Location::Register(Registers::Rsi),
            2 => Location::Register(Registers::Rdx),
            3 => Location::Register(Registers::Rcx),
            4 => Location::Register(Registers::R8),
            5 => Location::Register(Registers::R9),
            _ => Location::Stack(n - 6),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        assembly_flavour::{AssemblyInstruction, AssemblyInstructionParameter},
        calling_convention::SystemVAmd64Abi,
        memory_manager::{Location, MemoryManager},
        registers::Registers,
    };

    #[test]
    fn test_arguments_locations() {
        let locations = SystemVAmd64Abi::arguments_locations(&["a".into(), "b".into(), "c".into()]);
        assert_eq!(
            locations,
            [
                Location::Register(Registers::Rdi),
                Location::Register(Registers::Rsi),
                Location::Register(Registers::Rdx)
            ]
        );

        let locations = SystemVAmd64Abi::arguments_locations(&[
            "a".into(),
            "b".into(),
            "c".into(),
            "d".into(),
            "e".into(),
            "f".into(),
            "g".into(),
        ]);
        assert_eq!(
            locations,
            [
                Location::Register(Registers::Rdi),
                Location::Register(Registers::Rsi),
                Location::Register(Registers::Rdx),
                Location::Register(Registers::Rcx),
                Location::Register(Registers::R8),
                Location::Register(Registers::R9),
                Location::Stack(0),
            ]
        );
    }

    #[test]
    fn test_reallocations() {
        let arguments_locations = [
            Location::Register(Registers::Rdi),
            Location::Register(Registers::Rsi),
            Location::Register(Registers::Rdx),
        ];

        let mm = &mut MemoryManager {
            stack_position: 3,
            free_registers: vec![
                Registers::R9,
                Registers::R8,
                Registers::Rdi,
                Registers::Rsi,
                Registers::Rdx,
                Registers::Rcx,
                Registers::Rbx,
                Registers::Rax,
            ],
            next_locations: vec![
                Location::Register(Registers::Rcx),
                Location::Register(Registers::Rbx),
                Location::Register(Registers::Rax),
            ],
            reservations: HashMap::from([
                ("a".into(), Location::Register(Registers::Rdi)),
                ("b".into(), Location::Register(Registers::Rsi)),
                ("c".into(), Location::Register(Registers::Rdx)),
            ]),
        };
        let reallocations = SystemVAmd64Abi::relocations(mm, &arguments_locations);

        assert_eq!(
            reallocations,
            [
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(Registers::Rcx),
                        AssemblyInstructionParameter::Register(Registers::Rdi),
                    ],
                    "Move `c` to a free location".into(),
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(Registers::R8),
                        AssemblyInstructionParameter::Register(Registers::Rsi),
                    ],
                    "Move `b` to a free location".into(),
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(Registers::R9),
                        AssemblyInstructionParameter::Register(Registers::Rdx),
                    ],
                    "Move `a` to a free location".into(),
                ),
            ]
        );
    }
}
