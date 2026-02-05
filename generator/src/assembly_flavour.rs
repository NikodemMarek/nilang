use errors::GeneratorErrors;

use crate::registers::Registers;

pub trait AssemblyFlavour<R: Registers> {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String;
    fn generate_instruction(
        instruction: &AssemblyInstruction,
        parameters: &[AssemblyInstructionParameter<R>],
        comment: &str,
    ) -> String;

    fn generate_program_scaffold() -> Vec<String>;

    fn generate_function_header(name: &str) -> String;
    fn generate_function_body<'a>(
        instructions: impl Iterator<Item = Result<FullInstruction<R>, GeneratorErrors>> + 'a,
    ) -> impl Iterator<Item = Result<String, GeneratorErrors>> + 'a;
}

pub struct AtAndTFlavour;

impl<R: Registers> AssemblyFlavour<R> for AtAndTFlavour {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String {
        match parameter {
            AssemblyInstructionParameter::Register(register) => format!("%{register}"),
            AssemblyInstructionParameter::Memory(memory) => format!("-{memory}(%rax)"),
            AssemblyInstructionParameter::Number(number) => format!("${number}"),
            AssemblyInstructionParameter::Char(char) => format!("$'{char}'"),
            AssemblyInstructionParameter::Function(name) => name.to_string(),
            AssemblyInstructionParameter::Label(name) => format!(".{name}"),
            AssemblyInstructionParameter::Data(pointer) => format!("${pointer}"),
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
            AssemblyInstruction::Label => format!("{}:", parameters[0]).into(),
            AssemblyInstruction::Jmp => instruction_with_arguments("jmp", &[&parameters[0]]),
            AssemblyInstruction::Je => instruction_with_arguments("je", &[&parameters[0]]),
            AssemblyInstruction::Test => {
                instruction_with_arguments("testq", &[&parameters[1], &parameters[0]])
            }
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

            AssemblyInstruction::Raw(instruction) => format!("{instruction}").into(),
        };

        asm_with_comment(&instruction, comment).into()
    }

    fn generate_program_scaffold() -> Vec<String> {
        let mpty: [AssemblyInstructionParameter<R>; 0] = [];
        vec![
            Self::generate_instruction(&AssemblyInstruction::Raw(".text".into()), &mpty, ""),
            Self::generate_instruction(
                &AssemblyInstruction::Raw(".globl _start".into()),
                &mpty,
                "",
            ),
            Self::generate_instruction(&AssemblyInstruction::Raw("_start:".into()), &mpty, ""),
            Self::generate_instruction(&AssemblyInstruction::Raw("call main".into()), &mpty, ""),
            Self::generate_instruction(
                &AssemblyInstruction::Raw("movq $60, %rax".into()),
                &mpty,
                "",
            ),
            Self::generate_instruction(
                &AssemblyInstruction::Raw("xorq %rdi, %rdi".into()),
                &mpty,
                "",
            ),
            Self::generate_instruction(&AssemblyInstruction::Raw("syscall".into()), &mpty, ""),
        ]
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
    Label, // label
    Jmp,   // label
    Je,    // label
    Test,  // a, b
    Move,  // destination, source
    Swap,  // a, b
    Call,  // function
    Add,   // destination & a, b
    Sub,   // destination & a, b
    Mul,   // destination & a, b
    Div,   // destination & a

    Raw(Box<str>), //  TODO: Remove
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstructionParameter<R: Registers> {
    Register(R),
    Memory(usize),
    Number(f64),
    Char(char),
    Function(Box<str>),
    Label(Box<str>),
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
