use errors::GeneratorErrors;

use crate::{
    assembly_flavour::{AssemblyInstruction, AssemblyInstructionParameter, FullInstruction},
    memory_manager::{Location, MemoryManager},
    registers::{Registers, X86Registers},
};

pub trait CallingConvention {
    type R: Registers;

    fn generate_function_call(
        mm: &mut MemoryManager<Self::R>,
        name: &str,
        args: &[Box<str>],
        return_temporary: Option<Box<str>>,
    ) -> Result<Vec<FullInstruction<Self::R>>, GeneratorErrors>;

    fn return_location() -> Location<Self::R>;
    fn nth_argument_location(n: usize) -> Location<Self::R>;
}

pub struct SystemVAmd64Abi;

impl SystemVAmd64Abi {
    fn arguments_locations(arguments: &[Box<str>]) -> Vec<Location<X86Registers>> {
        let available_registers = [
            X86Registers::Rdi,
            X86Registers::Rsi,
            X86Registers::Rdx,
            X86Registers::Rcx,
            X86Registers::R8,
            X86Registers::R9,
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
        mm: &mut MemoryManager<X86Registers>,
        arguments_locations: &[Location<X86Registers>],
    ) -> Vec<FullInstruction<X86Registers>> {
        arguments_locations
            .iter()
            .filter_map(|loc| {
                if !mm.is_taken(loc) {
                    return None;
                }

                let var = mm.get_name(loc).unwrap().to_string();
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
    type R = X86Registers;

    fn generate_function_call(
        mm: &mut MemoryManager<Self::R>,
        name: &str,
        args: &[Box<str>],
        return_temporary: Option<Box<str>>,
    ) -> Result<Vec<FullInstruction<Self::R>>, GeneratorErrors> {
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
                let arg_loc = mm.get_location_or_err(arg).unwrap();
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

        let move_result = if let Some(return_temporary) = return_temporary {
            let return_register = mm.reserve(&return_temporary);
            [(
                AssemblyInstruction::Move,
                vec![
                    return_register.into(),
                    SystemVAmd64Abi::return_location().into(),
                ],
                format!("Move result of `{name}` to return register").into(),
            )]
            .into()
        } else {
            [].into()
        };

        Ok([
            relocations,
            arguments_allocations,
            stack_alignment.into(),
            function_call.into(),
            stack_cleanup.into(),
            move_result,
        ]
        .concat())
    }

    fn return_location() -> Location<Self::R> {
        Location::Register(X86Registers::Rax)
    }

    fn nth_argument_location(n: usize) -> Location<Self::R> {
        match n {
            0 => Location::Register(X86Registers::Rdi),
            1 => Location::Register(X86Registers::Rsi),
            2 => Location::Register(X86Registers::Rdx),
            3 => Location::Register(X86Registers::Rcx),
            4 => Location::Register(X86Registers::R8),
            5 => Location::Register(X86Registers::R9),
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
        registers::X86Registers,
    };

    #[test]
    fn test_arguments_locations() {
        let locations = SystemVAmd64Abi::arguments_locations(&["a".into(), "b".into(), "c".into()]);
        assert_eq!(
            locations,
            [
                Location::Register(X86Registers::Rdi),
                Location::Register(X86Registers::Rsi),
                Location::Register(X86Registers::Rdx)
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
                Location::Register(X86Registers::Rdi),
                Location::Register(X86Registers::Rsi),
                Location::Register(X86Registers::Rdx),
                Location::Register(X86Registers::Rcx),
                Location::Register(X86Registers::R8),
                Location::Register(X86Registers::R9),
                Location::Stack(0),
            ]
        );
    }

    #[test]
    fn test_reallocations() {
        let arguments_locations = [
            Location::Register(X86Registers::Rdi),
            Location::Register(X86Registers::Rsi),
            Location::Register(X86Registers::Rdx),
        ];

        let mm = &mut MemoryManager {
            stack_position: 3,
            free_registers: vec![
                X86Registers::R9,
                X86Registers::R8,
                X86Registers::Rdi,
                X86Registers::Rsi,
                X86Registers::Rdx,
                X86Registers::Rcx,
                X86Registers::Rbx,
                X86Registers::Rax,
            ],
            next_locations: vec![
                Location::Register(X86Registers::Rcx),
                Location::Register(X86Registers::Rbx),
                Location::Register(X86Registers::Rax),
            ],
            reservations: HashMap::from([
                ("a".into(), Location::Register(X86Registers::Rdi)),
                ("b".into(), Location::Register(X86Registers::Rsi)),
                ("c".into(), Location::Register(X86Registers::Rdx)),
            ]),
        };
        let reallocations = SystemVAmd64Abi::relocations(mm, &arguments_locations);

        assert_eq!(
            reallocations,
            [
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(X86Registers::Rcx),
                        AssemblyInstructionParameter::Register(X86Registers::Rdi),
                    ],
                    "Move `c` to a free location".into(),
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(X86Registers::R8),
                        AssemblyInstructionParameter::Register(X86Registers::Rsi),
                    ],
                    "Move `b` to a free location".into(),
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        AssemblyInstructionParameter::Register(X86Registers::R9),
                        AssemblyInstructionParameter::Register(X86Registers::Rdx),
                    ],
                    "Move `a` to a free location".into(),
                ),
            ]
        );
    }
}
