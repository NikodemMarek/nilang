mod assembly_flavour;
mod calling_convention;
mod memory_manager;
mod registers;

pub mod options {
    pub use crate::assembly_flavour::AtAndTFlavour;
    pub use crate::calling_convention::SystemVAmd64Abi;
    pub use crate::registers::X86Registers;
}

use std::iter::once;

use assembly_flavour::{
    AssemblyFlavour, AssemblyInstruction, AssemblyInstructionParameter, FullInstruction,
};
use calling_convention::CallingConvention;
use errors::GeneratorErrors;
use memory_manager::MemoryManager;
use nilang_types::instructions::Instruction;
use registers::{Registers, X86Registers};

pub fn generate_program<A>() -> impl Iterator<Item = String> + 'static
where
    A: AssemblyFlavour<X86Registers>,
{
    A::generate_program_scaffold()
}

pub fn generate_data<A>(data: &[(Box<str>, Box<str>)]) -> impl Iterator<Item = String> + '_
where
    A: AssemblyFlavour<X86Registers>,
{
    data.iter()
        .map(move |(name, value)| format!("{}: .asciz \"{}\"\n", name, value))
}

pub fn generate_function<'a, R, C, A>(
    name: Box<str>,
    data: &'a [Box<str>],
    instructions: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Result<String, GeneratorErrors>> + 'a
where
    R: Registers + 'a,
    C: CallingConvention<Registers = R>,
    A: AssemblyFlavour<R>,
{
    let header = A::generate_function_header(&name);
    let body =
        A::generate_function_body(generate_instructions::<R, C>(data, instructions)).map(|line| {
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

fn generate_instructions<'a, R, C>(
    data: &'a [Box<str>],
    instructions: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Result<FullInstruction<C::Registers>, GeneratorErrors>> + 'a
where
    R: Registers + 'a,
    C: CallingConvention<Registers = R>,
{
    let mut mm = MemoryManager::<R>::new(&[builtin_variables(), data.into()].concat());
    Box::new(instructions.flat_map(move |instruction| {
        let generated_instruction = C::generate_instruction(&mut mm, instruction);

        match generated_instruction {
            Ok(v) => v
                .into_iter()
                .map(
                    Ok::<
                        (
                            AssemblyInstruction,
                            Vec<AssemblyInstructionParameter<C::Registers>>,
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

fn builtin_functions<C: CallingConvention>(
    mm: &mut MemoryManager<C::Registers>,
    name: &str,
    arguments: &[Box<str>],
    _return_temporary: Option<Box<str>>,
) -> Option<Result<Vec<FullInstruction<C::Registers>>, GeneratorErrors>> {
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
        "print" => Some(C::generate_function_call(
            mm,
            "printf",
            &["print_format".into(), arguments.first().unwrap().clone()],
            None,
        )),
        _ => None,
    }
}

fn builtin_variables() -> Box<[Box<str>]> {
    Box::new([
        "printi_format".into(),
        "printc_format".into(),
        "print_format".into(),
    ])
}
