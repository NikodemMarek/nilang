use std::iter::zip;

use errors::GeneratorErrors;
use nilang_types::instructions::Instruction;

use crate::{
    assembly_flavour::{AssemblyInstruction, AssemblyInstructionParameter, FullInstruction},
    builtin_functions,
    memory_manager::{free_locations, Location, MemoryManager},
    registers::{Registers, X86Registers},
};

pub trait CallingConvention: Sized {
    type Registers: Registers;

    fn generate_function_call(
        mm: &mut MemoryManager<Self::Registers>,
        name: &str,
        args: &[Box<str>],
        return_temporary: Option<Box<str>>,
    ) -> Result<Vec<FullInstruction<Self::Registers>>, GeneratorErrors>;

    fn return_location() -> Location<Self::Registers>;
    fn nth_argument_location(n: usize) -> Location<Self::Registers>;

    fn arguments_locations(arguments: &[Box<str>]) -> Vec<Location<Self::Registers>> {
        arguments
            .iter()
            .enumerate()
            .map(|(i, _)| Self::nth_argument_location(i))
            .collect()
    }

    fn generate_instruction(
        mm: &mut MemoryManager<Self::Registers>,
        instruction: Instruction,
    ) -> Result<Vec<FullInstruction<Self::Registers>>, GeneratorErrors> {
        Ok(match instruction {
            Instruction::Declare(variable) => {
                mm.reserve(&variable)?;
                Vec::new()
            }
            Instruction::FunctionCall(name, arguments, return_temporary) => {
                if let Some(instructions) =
                    builtin_functions::<Self>(mm, &name, &arguments, return_temporary.clone())
                {
                    return instructions;
                }

                Self::generate_function_call(mm, &name, &arguments, return_temporary)?
            }
            Instruction::LoadBoolean(temporary, boolean) => {
                let location = mm.get_location_or_err(&temporary)?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![
                        location.into(),
                        AssemblyInstructionParameter::Number(if boolean { 1.0 } else { 0.0 }),
                    ],
                    format!("Load boolean '{boolean}' into `{temporary}`").into(),
                )]
            }
            Instruction::LoadNumber(temporary, number) => {
                let location = mm.get_location_or_err(&temporary)?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![
                        location.into(),
                        AssemblyInstructionParameter::Number(number),
                    ],
                    format!("Load number '{number}' into `{temporary}`").into(),
                )]
            }
            Instruction::LoadChar(temporary, character) => {
                let location = mm.get_location_or_err(&temporary)?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![
                        location.into(),
                        AssemblyInstructionParameter::Char(character),
                    ],
                    format!("Load character '{character}' into `{temporary}`").into(),
                )]
            }
            Instruction::LoadStringLocation(temporary, string) => {
                let pointer_location = mm.get_location_or_err(&temporary)?;
                let string_location = mm.get_location_or_err(&string)?;

                vec![(
                    AssemblyInstruction::Move,
                    vec![pointer_location.into(), string_location.into()],
                    format!("Load '{string}' string pointer into `{temporary}`").into(),
                )]
            }
            Instruction::ReturnVariable(temporary) => {
                let location = mm.get_location_or_err(&temporary)?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![Self::return_location().into(), location.into()],
                    format!("Return `{temporary}`").into(),
                )]
            }
            Instruction::TakeArgument(argument, temporary) => {
                let location = Self::nth_argument_location(argument);
                mm.reserve_location(&temporary, location.clone())?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![location.clone().into(), location.into()],
                    format!("Load `{temporary}` as argument {argument}").into(),
                )]
            }
            Instruction::Copy(to, from) => {
                let from_loc = mm.get_location_or_err(&from)?;
                let to_loc = mm.get_location_or_err(&to)?;
                vec![(
                    AssemblyInstruction::Move,
                    vec![to_loc.into(), from_loc.into()],
                    format!("Copy `{from}` into `{to}`").into(),
                )]
            }
            Instruction::AddVariables(result, a, b) => {
                let a_loc = mm.get_location_or_err(&a)?;
                let b_loc = mm.get_location_or_err(&b)?;
                let result_loc = mm.get_location_or_err(&result)?;
                vec![
                    (
                        AssemblyInstruction::Move,
                        vec![result_loc.into(), a_loc.into()],
                        format!("Prepare `{result}` for addition").into(),
                    ),
                    (
                        AssemblyInstruction::Add,
                        vec![result_loc.into(), b_loc.into()],
                        format!("Add `{a}` and `{b}` into `{result}`").into(),
                    ),
                ]
            }
            Instruction::SubtractVariables(result, a, b) => {
                let a_loc = mm.get_location_or_err(&a)?;
                let b_loc = mm.get_location_or_err(&b)?;
                let result_loc = mm.get_location_or_err(&result)?;
                vec![
                    (
                        AssemblyInstruction::Move,
                        vec![result_loc.into(), a_loc.into()],
                        format!("Prepare `{result}` for subtraction").into(),
                    ),
                    (
                        AssemblyInstruction::Sub,
                        vec![result_loc.into(), b_loc.into()],
                        format!("Subtract `{b}` from `{result}`").into(),
                    ),
                ]
            }
            Instruction::MultiplyVariables(result, a, b) => {
                let a_loc = mm.get_location_or_err(&a)?;
                let b_loc = mm.get_location_or_err(&b)?;
                let result_loc = mm.get_location_or_err(&result)?;
                vec![
                    (
                        AssemblyInstruction::Move,
                        vec![result_loc.into(), a_loc.into()],
                        format!("Prepare `{result}` for multiplication").into(),
                    ),
                    (
                        AssemblyInstruction::Mul,
                        vec![result_loc.into(), b_loc.into()],
                        format!("Multiply `{a}` and `{b}` into `{result}`").into(),
                    ),
                ]
            }
            Instruction::DivideVariables(_, _, _) | Instruction::ModuloVariables(_, _, _) => {
                Self::generate_instruction_specific(mm, instruction)?
            }
            Instruction::Label(label) => {
                vec![(
                    AssemblyInstruction::Label,
                    vec![AssemblyInstructionParameter::Label(label.clone())],
                    format!("Create label `{label}`").into(),
                )]
            }
            Instruction::ConditionalJump(check, label) => {
                let check_loc = mm.get_location_or_err(&check)?;
                vec![
                    (
                        AssemblyInstruction::Test,
                        vec![check_loc.into(), check_loc.into()],
                        format!("Test if `{check}` is `0`").into(),
                    ),
                    (
                        AssemblyInstruction::Je,
                        vec![AssemblyInstructionParameter::Label(label.clone())],
                        format!("Jump to label `{label}` if `{check}` test passed").into(),
                    ),
                ]
            }
        })
    }

    fn generate_instruction_specific(
        mm: &mut MemoryManager<Self::Registers>,
        instruction: Instruction,
    ) -> Result<Vec<FullInstruction<Self::Registers>>, GeneratorErrors>;
}

pub struct SystemVAmd64Abi;

impl CallingConvention for SystemVAmd64Abi {
    type Registers = X86Registers;

    fn generate_function_call(
        mm: &mut MemoryManager<Self::Registers>,
        name: &str,
        arguments: &[Box<str>],
        return_temporary: Option<Box<str>>,
    ) -> Result<Vec<FullInstruction<Self::Registers>>, GeneratorErrors> {
        let arguments_locations = Self::arguments_locations(arguments);
        let arguments_allocations = allocate_in(mm, arguments, &arguments_locations)?;

        let stack_alignment = [];

        let function_call = [(
            AssemblyInstruction::Call,
            vec![AssemblyInstructionParameter::Function(name.into())],
            format!("Call function `{name}`").into(),
        )];

        let stack_cleanup = [];

        let move_result = if let Some(return_temporary) = return_temporary {
            let return_register = mm.get_location_or_err(&return_temporary)?;
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

        for i in 0..arguments.len() {
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

    fn return_location() -> Location<Self::Registers> {
        Location::Register(X86Registers::Rax)
    }

    fn nth_argument_location(n: usize) -> Location<Self::Registers> {
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

    fn generate_instruction_specific(
        mm: &mut MemoryManager<X86Registers>,
        instruction: Instruction,
    ) -> Result<Vec<FullInstruction<X86Registers>>, GeneratorErrors> {
        Ok(match instruction {
            Instruction::DivideVariables(result, divident, divisor) => {
                let empty_temp = "@empty".to_string();
                mm.reserve(&empty_temp)?;
                let mut alloc = allocate_in(
                    mm,
                    &[divident.clone(), empty_temp.clone().into()],
                    &[
                        Location::Register(X86Registers::Rax),
                        Location::Register(X86Registers::Rdx),
                    ],
                )?;
                mm.free(&empty_temp);

                alloc.append(&mut vec![
                    (
                        AssemblyInstruction::Move,
                        vec![
                            Location::Register(X86Registers::Rdx).into(),
                            AssemblyInstructionParameter::Number(0f64),
                        ],
                        format!("Prepare `{result}` for division").into(),
                    ),
                    (
                        AssemblyInstruction::Move,
                        vec![
                            Location::Register(X86Registers::Rax).into(),
                            mm.get_location_or_err(&divident)?.into(),
                        ],
                        format!("Prepare `{result}` for division").into(),
                    ),
                    (
                        AssemblyInstruction::Div,
                        vec![mm.get_location_or_err(&divisor)?.into()],
                        format!("Divide `{divident}` by `{divisor}`").into(),
                    ),
                    (
                        AssemblyInstruction::Move,
                        vec![
                            mm.get_location_or_err(&result)?.into(),
                            Location::Register(X86Registers::Rax).into(),
                        ],
                        format!("Move result of division into `{result}`").into(),
                    ),
                ]);

                alloc
            }
            Instruction::ModuloVariables(result, divident, divisor) => {
                let empty_temp = "@empty".to_string();
                mm.reserve(&empty_temp)?;
                let mut alloc = allocate_in(
                    mm,
                    &[divident.clone(), empty_temp.clone().into()],
                    &[
                        Location::Register(X86Registers::Rax),
                        Location::Register(X86Registers::Rdx),
                    ],
                )?;
                mm.free(&empty_temp);

                alloc.append(&mut vec![
                    (
                        AssemblyInstruction::Move,
                        vec![
                            Location::Register(X86Registers::Rdx).into(),
                            AssemblyInstructionParameter::Number(0f64),
                        ],
                        format!("Prepare `{result}` for modulo").into(),
                    ),
                    (
                        AssemblyInstruction::Move,
                        vec![
                            Location::Register(X86Registers::Rax).into(),
                            mm.get_location_or_err(&divident)?.into(),
                        ],
                        format!("Prepare `{result}` for modulo").into(),
                    ),
                    (
                        AssemblyInstruction::Div,
                        vec![mm.get_location_or_err(&divisor)?.into()],
                        format!("Divide `{divident}` by `{divisor}`").into(),
                    ),
                    (
                        AssemblyInstruction::Move,
                        vec![
                            mm.get_location_or_err(&result)?.into(),
                            Location::Register(X86Registers::Rdx).into(),
                        ],
                        format!("Move result of modulo into `{result}`").into(),
                    ),
                ]);
                alloc
            }
            _ => unreachable!(),
        })
    }
}

pub fn allocate_in<R: Registers>(
    mm: &mut MemoryManager<R>,
    temporaries: &[Box<str>],
    locations: &[Location<R>],
) -> Result<Vec<FullInstruction<R>>, GeneratorErrors> {
    let free = free_locations(mm, locations)?;
    let allocations = zip(temporaries.iter(), locations.iter())
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

    for i in 0..temporaries.len() {
        mm.free(&format!("@arg_{i}"));
    }

    Ok([free, allocations].concat())
}

#[cfg(test)]
mod tests {
    use crate::{
        assembly_flavour::AssemblyInstruction,
        calling_convention::allocate_in,
        memory_manager::{Location, MemoryManager},
        registers::tests::TestRegisters,
    };

    use super::CallingConvention;

    struct TestCallingConvention;
    impl CallingConvention for TestCallingConvention {
        type Registers = TestRegisters;

        fn generate_function_call(
            _mm: &mut MemoryManager<Self::Registers>,
            _name: &str,
            _args: &[Box<str>],
            _return_temporary: Option<Box<str>>,
        ) -> Result<
            Vec<crate::assembly_flavour::FullInstruction<Self::Registers>>,
            errors::GeneratorErrors,
        > {
            unreachable!()
        }

        fn return_location() -> Location<Self::Registers> {
            Location::Register(TestRegisters::R(0))
        }

        fn nth_argument_location(n: usize) -> Location<Self::Registers> {
            if n == 0 {
                Location::Register(TestRegisters::R(1))
            } else if n == 1 {
                Location::Register(TestRegisters::R(2))
            } else {
                Location::Stack(n - 2)
            }
        }

        fn generate_instruction_specific(
            _mm: &mut MemoryManager<Self::Registers>,
            _instruction: nilang_types::instructions::Instruction,
        ) -> Result<
            Vec<crate::assembly_flavour::FullInstruction<Self::Registers>>,
            errors::GeneratorErrors,
        > {
            unreachable!()
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
        let arguments_locations = TestCallingConvention::arguments_locations(&arguments);
        let instructions = allocate_in(&mut mm, &arguments, &arguments_locations).unwrap();

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
