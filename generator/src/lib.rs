mod assembly_flavour;
mod calling_convention;
mod memory_manager;
mod registers;
mod utils;

pub mod options {
    pub use crate::assembly_flavour::AtAndTFlavour;
    pub use crate::calling_convention::SystemVAmd64Abi;
}

use std::collections::HashMap;

use assembly_flavour::{
    AssemblyFlavour, AssemblyInstruction, AssemblyInstructionParameter, FullInstruction,
};
use calling_convention::CallingConvention;
use errors::GeneratorErrors;
use memory_manager::MemoryManager;
use nilang_types::instructions::Instruction;
use registers::X86Registers;

pub fn generate<C, A>(
    functions: HashMap<Box<str>, Vec<Instruction>>,
) -> Result<Box<str>, GeneratorErrors>
where
    C: CallingConvention<R = X86Registers>,
    A: AssemblyFlavour<C::R>,
{
    let mut code = Vec::new();

    for (name, instructions) in functions.into_iter() {
        code.push(A::generate_function(
            &name,
            &generate_function::<C>(&instructions)?,
        ));
    }

    Ok(A::generate_program(&code))
}

fn generate_function<C>(code: &[Instruction]) -> Result<Vec<FullInstruction<C::R>>, GeneratorErrors>
where
    C: CallingConvention<R = X86Registers>,
{
    let mm = &mut MemoryManager::default();
    code.iter().try_fold(Vec::new(), |mut acc, instruction| {
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
        Instruction::FunctionCall(name, arguments, return_temporary) => {
            if let Some(instructions) =
                builtin_functions::<C>(mm, &name, &arguments, return_temporary.clone())
            {
                return instructions;
            }

            C::generate_function_call(mm, &name, &arguments, return_temporary)?
        }
        Instruction::LoadNumber(temporary, number) => {
            let location = mm.reserve(&temporary);
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
            let location = mm.reserve(&temporary);
            vec![(
                AssemblyInstruction::Move,
                vec![
                    location.into(),
                    AssemblyInstructionParameter::Char(character),
                ],
                format!("Load character '{character}' into `{temporary}`").into(),
            )]
        }
        Instruction::ReturnVariable(variable) => {
            let location = mm.get_location(&variable).unwrap().clone();
            vec![(
                AssemblyInstruction::Move,
                vec![C::return_location().into(), location.into()],
                format!("Return `{variable}`").into(),
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
            let from_loc = mm.get_location(&from).unwrap().clone();
            let to_loc = mm.reserve(&to);
            vec![(
                AssemblyInstruction::Move,
                vec![to_loc.into(), from_loc.into()],
                format!("Copy `{from}` into `{to}`").into(),
            )]
        }
        Instruction::AddVariables(result, a, b) => {
            let a_loc = mm.get_location(&a).unwrap().clone();
            let b_loc = mm.get_location(&b).unwrap().clone();
            let result_loc = mm.reserve(&result);
            vec![
                (
                    AssemblyInstruction::Move,
                    vec![result_loc.clone().into(), a_loc.clone().into()],
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
            let a_loc = mm.get_location(&a).unwrap().clone();
            let b_loc = mm.get_location(&b).unwrap().clone();
            let result_loc = mm.reserve(&result);
            vec![
                (
                    AssemblyInstruction::Move,
                    vec![result_loc.clone().into(), a_loc.clone().into()],
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
            let a_loc = mm.get_location(&a).unwrap().clone();
            let b_loc = mm.get_location(&b).unwrap().clone();
            let result_loc = mm.reserve(&result);
            vec![
                (
                    AssemblyInstruction::Move,
                    vec![result_loc.clone().into(), a_loc.clone().into()],
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
    return_temporary: Box<str>,
) -> Option<Result<Vec<FullInstruction<C::R>>, GeneratorErrors>> {
    match name {
        "printi" => Some(C::generate_function_call(
            mm,
            "printf",
            &["printi_format".into(), arguments.first().unwrap().clone()],
            return_temporary,
        )),
        "printc" => Some(C::generate_function_call(
            mm,
            "printf",
            &["printc_format".into(), arguments.first().unwrap().clone()],
            return_temporary,
        )),
        _ => None,
    }
}
