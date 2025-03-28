use std::iter::zip;

use errors::GeneratorErrors;

use crate::{
    assembly_flavour::{AssemblyInstruction, AssemblyInstructionParameter, FullInstruction},
    memory_manager::{free_locations, Location, MemoryManager},
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

    fn arguments_locations(arguments: &[Box<str>]) -> Vec<Location<Self::R>> {
        arguments
            .iter()
            .enumerate()
            .map(|(i, _)| Self::nth_argument_location(i))
            .collect()
    }

    fn arguments_allocations(
        mm: &mut MemoryManager<Self::R>,
        arguments: &[Box<str>],
    ) -> Result<Vec<FullInstruction<Self::R>>, GeneratorErrors> {
        let arguments_locations = Self::arguments_locations(arguments);

        let free = free_locations(mm, &arguments_locations)?;
        let allocations = zip(arguments.iter(), arguments_locations.iter())
            .enumerate()
            .map(|(i, (arg_temp, desired_arg_location))| {
                let current_arg_loc = mm.get_location_or_err(arg_temp)?.to_owned();
                mm.reserve_location(&format!("@arg_{i}"), desired_arg_location.clone())?;
                Ok((
                    AssemblyInstruction::Move,
                    vec![desired_arg_location.into(), current_arg_loc.clone().into()],
                    format!("Load `{arg_temp}` as argument {i}").into(),
                ))
            })
            .collect::<Result<Vec<_>, GeneratorErrors>>()?;

        for i in 0..arguments.len() {
            mm.free(&format!("@arg_{i}"));
        }

        Ok([free, allocations].concat())
    }
}

pub struct SystemVAmd64Abi;

impl CallingConvention for SystemVAmd64Abi {
    type R = X86Registers;

    fn generate_function_call(
        mm: &mut MemoryManager<Self::R>,
        name: &str,
        args: &[Box<str>],
        return_temporary: Option<Box<str>>,
    ) -> Result<Vec<FullInstruction<Self::R>>, GeneratorErrors> {
        let arguments_allocations = Self::arguments_allocations(mm, args)?;

        let stack_alignment = [];

        let function_call = [(
            AssemblyInstruction::Call,
            vec![AssemblyInstructionParameter::Function(name.into())],
            format!("Call function `{name}`").into(),
        )];

        let stack_cleanup = [];

        let move_result = if let Some(return_temporary) = return_temporary {
            let return_register = mm.reserve(&return_temporary).unwrap();
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

        for i in 0..args.len() {
            mm.free(&format!("arg_{i}"));
        }

        Ok([
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
    use crate::{
        assembly_flavour::AssemblyInstruction,
        memory_manager::{Location, MemoryManager},
        registers::tests::TestRegisters,
    };

    use super::CallingConvention;

    struct TestCallingConvention;
    impl CallingConvention for TestCallingConvention {
        type R = TestRegisters;

        fn generate_function_call(
            _mm: &mut MemoryManager<Self::R>,
            _name: &str,
            _args: &[Box<str>],
            _return_temporary: Option<Box<str>>,
        ) -> Result<Vec<crate::assembly_flavour::FullInstruction<Self::R>>, errors::GeneratorErrors>
        {
            unreachable!()
        }

        fn return_location() -> Location<Self::R> {
            Location::Register(TestRegisters::R(0))
        }

        fn nth_argument_location(n: usize) -> Location<Self::R> {
            if n == 0 {
                Location::Register(TestRegisters::R(1))
            } else if n == 1 {
                Location::Register(TestRegisters::R(2))
            } else {
                Location::Stack(n - 2)
            }
        }
    }

    #[test]
    fn test_arguments_locations() {
        let arguments = ["a".into(), "b".into(), "c".into()];
        let arguments_locations = TestCallingConvention::arguments_locations(&arguments);

        assert_eq!(
            arguments_locations,
            [
                Location::Register(TestRegisters::R(1)),
                Location::Register(TestRegisters::R(2)),
                Location::Stack(0),
            ]
        );
    }

    #[test]
    fn arguments_allocations() {
        let mut mm = MemoryManager::<TestRegisters>::default();
        mm.reserve("a").unwrap();
        mm.reserve("b").unwrap();
        mm.reserve("c").unwrap();

        let arguments = ["a".into(), "b".into(), "c".into()];
        let instructions =
            TestCallingConvention::arguments_allocations(&mut mm, &arguments).unwrap();

        assert_eq!(
            instructions,
            [
                (
                    AssemblyInstruction::Swap,
                    vec![
                        Location::Register(TestRegisters::R(1)).into(),
                        Location::Stack(0).into()
                    ],
                    "Swap b and @swap_temp_0".into()
                ),
                (
                    AssemblyInstruction::Swap,
                    vec![
                        Location::Register(TestRegisters::R(2)).into(),
                        Location::Stack(1).into()
                    ],
                    "Swap c and @swap_temp_1".into()
                ),
                (
                    AssemblyInstruction::Swap,
                    vec![Location::Stack(0).into(), Location::Stack(2).into()],
                    "Swap b and @swap_temp_2".into()
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        Location::Register(TestRegisters::R(1)).into(),
                        Location::Register(TestRegisters::R(0)).into()
                    ],
                    "Load `a` as argument 0".into()
                ),
                (
                    AssemblyInstruction::Move,
                    vec![
                        Location::Register(TestRegisters::R(2)).into(),
                        Location::Stack(2).into()
                    ],
                    "Load `b` as argument 1".into()
                ),
                (
                    AssemblyInstruction::Move,
                    vec![Location::Stack(0).into(), Location::Stack(1).into()],
                    "Load `c` as argument 2".into()
                )
            ]
        );
    }
}
