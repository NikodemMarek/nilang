mod assembly_flavour;
mod calling_convention;
mod memory_manager;
mod registers;

pub mod options {
    pub use crate::assembly_flavour::AtAndTFlavour;
    pub use crate::calling_convention::SystemVAmd64Abi;
}

use std::iter::once;

use assembly_flavour::{
    AssemblyFlavour, AssemblyInstruction, AssemblyInstructionParameter, FullInstruction,
};
use calling_convention::{allocate_in, CallingConvention};
use errors::GeneratorErrors;
use memory_manager::{Location, MemoryManager};
use nilang_types::instructions::Instruction;
use registers::X86Registers;

pub fn generate_program<A>() -> impl Iterator<Item = String> + 'static
where
    A: AssemblyFlavour<X86Registers>,
{
    A::generate_program_scaffold()
}

pub fn generate_function<'a, C, A>(
    name: Box<str>,
    instructions: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Result<String, GeneratorErrors>> + 'a
where
    C: CallingConvention<R = X86Registers>,
    A: AssemblyFlavour<C::R>,
{
    let header = A::generate_function_header(&name);
    let body = A::generate_function_body(generate_instructions::<C>(instructions)).map(|line| {
        line.map(|line| {
            let line = line.trim();
            if line.is_empty() {
                "".to_owned()
            } else {
                format!("    {}\n", line)
            }
        })
    });

    once(Ok(header)).chain(body)
}

fn generate_instructions<'a, C>(
    instructions: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Result<FullInstruction<C::R>, GeneratorErrors>> + 'a
where
    C: CallingConvention<R = X86Registers>,
{
    let mut mm = MemoryManager::default();
    Box::new(instructions.flat_map(move |instruction| {
        let generated_instruction = match instruction {
            Instruction::DivideVariables(_, _, _) | Instruction::ModuloVariables(_, _, _) => {
                generate_instruction_specific(&mut mm, instruction)
            }
            _ => generate_instruction::<C>(&mut mm, instruction),
        };

        match generated_instruction {
            Ok(v) => v
                .into_iter()
                .map(
                    Ok::<
                        (
                            AssemblyInstruction,
                            Vec<AssemblyInstructionParameter<X86Registers>>,
                            Box<str>,
                        ),
                        GeneratorErrors,
                    >,
                )
                .collect(),
            Err(e) => vec![Err(e)],
        }
    }))
}

fn generate_instruction<C>(
    mm: &mut MemoryManager<C::R>,
    instruction: Instruction,
) -> Result<Vec<FullInstruction<C::R>>, GeneratorErrors>
where
    C: CallingConvention,
{
    Ok(match instruction {
        Instruction::Declare(variable) => {
            mm.reserve(&variable)?;
            Vec::new()
        }
        Instruction::FunctionCall(name, arguments, return_temporary) => {
            if let Some(instructions) =
                builtin_functions::<C>(mm, &name, &arguments, return_temporary.clone())
            {
                return instructions;
            }

            C::generate_function_call(mm, &name, &arguments, return_temporary)?
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
        Instruction::ReturnVariable(temporary) => {
            let location = mm.get_location_or_err(&temporary)?;
            vec![(
                AssemblyInstruction::Move,
                vec![C::return_location().into(), location.into()],
                format!("Return `{temporary}`").into(),
            )]
        }
        Instruction::TakeArgument(argument, temporary) => {
            let location = C::nth_argument_location(argument);
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
            unreachable!()
        }
    })
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

fn builtin_functions<C: CallingConvention>(
    mm: &mut MemoryManager<C::R>,
    name: &str,
    arguments: &[Box<str>],
    _return_temporary: Option<Box<str>>,
) -> Option<Result<Vec<FullInstruction<C::R>>, GeneratorErrors>> {
    match name {
        "printi" => Some(C::generate_function_call(
            mm,
            "printf",
            &["printi_format".into(), arguments.first().unwrap().clone()],
            None,
        )),
        "printc" => Some(C::generate_function_call(
            mm,
            "printf",
            &["printc_format".into(), arguments.first().unwrap().clone()],
            None,
        )),
        _ => None,
    }
}
