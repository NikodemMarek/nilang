use crate::registers::Registers;

pub trait AssemblyFlavour<R: Registers> {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String;
    fn generate_instruction(
        instruction: &AssemblyInstruction,
        parameters: &[AssemblyInstructionParameter<R>],
        comment: &str,
    ) -> String;

    fn generate_program(functions: &[Box<str>]) -> Box<str>;
    fn generate_function(name: &str, instructions: &[FullInstruction<R>]) -> Box<str>;
}

pub struct AtAndTFlavour;

impl<R: Registers> AssemblyFlavour<R> for AtAndTFlavour {
    fn generate_parameter(parameter: &AssemblyInstructionParameter<R>) -> String {
        match parameter {
            AssemblyInstructionParameter::Register(register) => format!("%{}", register.name()),
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

    fn generate_program(functions: &[Box<str>]) -> Box<str> {
        let data_section = r#"
printi_format: .asciz "%d\n"
printc_format: .asciz "%c\n"
"#;
        let start_fn = r#"
.globl _start
_start:
    call main
    movq $60, %rax
    xorq %rdi, %rdi
    syscall
        "#;

        format!(
            ".data{}\n.text{}\n{}",
            data_section,
            start_fn,
            functions.join("\n\n")
        )
        .into()
    }

    fn generate_function(name: &str, instructions: &[FullInstruction<R>]) -> Box<str> {
        let function_declaration = format!(".globl {}\n{}:", name, name);
        let prologue = r#"
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        "#;
        let epilogue = r#"
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        "#;

        let code = std::convert::Into::<Box<str>>::into(
            instructions
                .iter()
                .map(|(instruction, parameters, comment)| {
                    format!(
                        "    {}",
                        Self::generate_instruction(instruction, parameters, comment)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

        format!(
            "{}\n{}\n{}\n{}",
            function_declaration, prologue, code, epilogue
        )
        .into()
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
                AssemblyInstructionParameter::Register(register.clone())
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
