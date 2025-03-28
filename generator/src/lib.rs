mod assembly_flavour;
mod calling_convention;
mod memory_manager;
mod registers;
mod utils;

pub mod options {
    pub use crate::assembly_flavour::AtAndTFlavour;
    pub use crate::calling_convention::SystemVAmd64Abi;
}

use assembly_flavour::{
    AssemblyFlavour, AssemblyInstruction, AssemblyInstructionParameter, FullInstruction,
};
use calling_convention::CallingConvention;
use errors::GeneratorErrors;
use memory_manager::MemoryManager;
use nilang_types::instructions::Instruction;
use registers::X86Registers;

pub fn generate_program<A>(functions: impl Iterator<Item = Box<str>>) -> Box<str>
where
    A: AssemblyFlavour<X86Registers>,
{
    A::generate_program(&functions.collect::<Vec<_>>())
}

pub fn generate_function<C, A>(
    name: Box<str>,
    instructions: impl Iterator<Item = Instruction>,
) -> Result<Box<str>, GeneratorErrors>
where
    C: CallingConvention<R = X86Registers>,
    A: AssemblyFlavour<C::R>,
{
    Ok(A::generate_function(
        &name,
        &generate_instructions::<C>(instructions)?,
    ))
}

fn generate_instructions<C>(
    mut instructions: impl Iterator<Item = Instruction>,
) -> Result<Vec<FullInstruction<C::R>>, GeneratorErrors>
where
    C: CallingConvention<R = X86Registers>,
{
    let mm = &mut MemoryManager::default();
    instructions.try_fold(Vec::new(), |mut acc, instruction| {
        acc.extend(generate_instruction::<C>(mm, instruction.clone())?);
        Ok(acc)
    })
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
        Instruction::LoadArgument(argument, temporary) => {
            let location = C::nth_argument_location(argument);
            mm.reserve_location(&temporary, location.clone());
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
            todo!()
        }
    })
}

fn builtin_functions<C: CallingConvention>(
    mm: &mut MemoryManager<C::R>,
    name: &str,
    arguments: &[Box<str>],
    return_temporary: Option<Box<str>>,
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
