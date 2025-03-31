use errors::GeneratorErrors;

use crate::registers::Registers;

pub trait AssemblyFlavour<R: Registers> {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String;
    fn generate_instruction(
        instruction: &AssemblyInstruction,
        parameters: &[AssemblyInstructionParameter<R>],
        comment: &str,
    ) -> String;

    fn generate_program_scaffold() -> impl Iterator<Item = String> + 'static;

    fn generate_function_header(name: &str) -> String;
    fn generate_function_body<'a>(
        instructions: impl Iterator<Item = Result<FullInstruction<R>, GeneratorErrors>> + 'a,
    ) -> impl Iterator<Item = Result<String, GeneratorErrors>> + 'a;
}

pub struct AtAndTFlavour;

impl<R: Registers> AssemblyFlavour<R> for AtAndTFlavour {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String {
        match parameter {
            AssemblyInstructionParameter::Register(register) => format!("%{}", register),
            AssemblyInstructionParameter::Memory(memory) => format!("-{}(%rax)", memory),
            AssemblyInstructionParameter::Number(number) => format!("${}", number),
            AssemblyInstructionParameter::Char(char) => format!("$'{}'", char),
            AssemblyInstructionParameter::Function(name) => name.to_string(),
            AssemblyInstructionParameter::Data(pointer) => format!("${}", pointer),
        }
    }

    fn generate_instruction(
        instruction: &AssemblyInstruction,
        parameters: &[AssemblyInstructionParameter<R>],
        comment: &str,
    ) -> String {
        let parameters = parameters
            .iter()
            .map(Self::generate_parameter)
            .collect::<Vec<_>>();

        let instruction = match instruction {
            AssemblyInstruction::Move => {
                instruction_with_arguments("movq", &[&parameters[1], &parameters[0]])
            }
            AssemblyInstruction::Swap => {
                instruction_with_arguments("xchgq", &[&parameters[0], &parameters[1]])
            }
            AssemblyInstruction::Call => instruction_with_arguments("call", &[&parameters[0]]),
            AssemblyInstruction::Add => {
                instruction_with_arguments("addq", &[&parameters[1], &parameters[0]])
            }
            AssemblyInstruction::Sub => {
                instruction_with_arguments("subq", &[&parameters[1], &parameters[0]])
            }
            AssemblyInstruction::Mul => {
                instruction_with_arguments("imulq", &[&parameters[1], &parameters[0]])
            }
            AssemblyInstruction::Div => instruction_with_arguments("idivq", &[&parameters[0]]),
        };

        asm_with_comment(&instruction, comment).into()
    }

    fn generate_program_scaffold() -> impl Iterator<Item = String> + 'static {
        let data_section = r#"
.data
printi_format: .asciz "%d\n"
printc_format: .asciz "%c\n"
"#;
        let start_fn = r#"
.text
.globl _start
_start:
    call main
    movq $60, %rax
    xorq %rdi, %rdi
    syscall
        "#;

        data_section
            .lines()
            .chain(start_fn.lines())
            .map(ToOwned::to_owned)
    }

    fn generate_function_header(name: &str) -> String {
        format!(".globl {name}\n{name}:\n")
    }

    fn generate_function_body<'a>(
        instructions: impl Iterator<Item = Result<FullInstruction<R>, GeneratorErrors>> + 'a,
    ) -> impl Iterator<Item = Result<String, GeneratorErrors>> + 'a {
        let prologue = r#"
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        "#;
        let body = instructions.map(|v| {
            v.map(|(instruction, parameters, comment)| {
                Self::generate_instruction(&instruction, &parameters, &comment)
            })
            .map(|v| format!("    {}", v))
        });
        let epilogue = r#"
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        "#;

        prologue
            .lines()
            .map(ToOwned::to_owned)
            .map(Ok)
            .chain(body)
            .chain(epilogue.lines().map(ToOwned::to_owned).map(Ok))
    }
}

fn instruction_with_arguments(instruction: &str, arguments: &[&str]) -> Box<str> {
    format!("{} {}", instruction, arguments.join(", ")).into()
}

fn asm_with_comment(asm: &str, comment: &str) -> Box<str> {
    format!("{:<29} # {comment}", asm).into()
}

pub type FullInstruction<R> = (
    AssemblyInstruction,
    Vec<AssemblyInstructionParameter<R>>,
    Box<str>,
);

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstruction {
    Move, // destination, source
    Swap, // a, b
    Call, // function
    Add,  // destination & a, b
    Sub,  // destination & a, b
    Mul,  // destination & a, b
    Div,  // destination & a
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstructionParameter<R: Registers> {
    Register(R),
    Memory(usize),
    Number(f64),
    Char(char),
    Function(Box<str>),
    Data(Box<str>),
}

impl<R: Registers> From<crate::memory_manager::Location<R>> for AssemblyInstructionParameter<R> {
    fn from(val: crate::memory_manager::Location<R>) -> Self {
        match val {
            crate::memory_manager::Location::Register(register) => {
                AssemblyInstructionParameter::Register(register)
            }
            crate::memory_manager::Location::Stack(offset) => {
                AssemblyInstructionParameter::Memory(offset)
            }
            crate::memory_manager::Location::Hardcoded(hardcoded) => {
                AssemblyInstructionParameter::Data(hardcoded)
            }
        }
    }
}
impl<R: Registers> From<&crate::memory_manager::Location<R>> for AssemblyInstructionParameter<R> {
    fn from(val: &crate::memory_manager::Location<R>) -> Self {
        match val {
            crate::memory_manager::Location::Register(register) => {
                AssemblyInstructionParameter::Register(*register)
            }
            crate::memory_manager::Location::Stack(offset) => {
                AssemblyInstructionParameter::Memory(*offset)
            }
            crate::memory_manager::Location::Hardcoded(hardcoded) => {
                AssemblyInstructionParameter::Data(hardcoded.clone())
            }
        }
    }
}
