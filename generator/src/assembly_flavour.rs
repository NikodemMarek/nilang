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
            AssemblyInstructionParameter::Function(name) => name.to_string(),
        }
    }

    fn generate_instruction(
        instruction: &AssemblyInstruction,
        parameters: &[AssemblyInstructionParameter<R>],
        comment: &str,
    ) -> String {
        asm_with_comment(
            &match instruction {
                AssemblyInstruction::Move => format!(
                    "movq {}, {}",
                    Self::generate_parameter(&parameters[1]),
                    Self::generate_parameter(&parameters[0])
                ),
                AssemblyInstruction::Call => {
                    format!("call _{}", Self::generate_parameter(&parameters[0]))
                }
                AssemblyInstruction::Add => format!(
                    "addq {}, {}",
                    Self::generate_parameter(&parameters[1]),
                    Self::generate_parameter(&parameters[0])
                ),
                AssemblyInstruction::Sub => format!(
                    "subq {}, {}",
                    Self::generate_parameter(&parameters[1]),
                    Self::generate_parameter(&parameters[0])
                ),
                AssemblyInstruction::Mul => format!(
                    "imulq {}, {}",
                    Self::generate_parameter(&parameters[1]),
                    Self::generate_parameter(&parameters[0])
                ),
                AssemblyInstruction::Div => {
                    format!("idivq {}", Self::generate_parameter(&parameters[0]))
                }
            },
            comment,
        )
        .into()
    }

    fn generate_program(functions: &[Box<str>]) -> Box<str> {
        let start_fn = r#"
.globl _start
_start:
    call _main
    # movq $60, %rax
    # xorq %rdi, %rdi
    # syscall
    movq %rax, %rbx
    movq $1, %rax
    int $0x80
    ret
        "#;

        format!(".text\n{}\n{}", start_fn, functions.join("\n\n")).into()
    }

    fn generate_function(name: &str, instructions: &[FullInstruction<R>]) -> Box<str> {
        let function_declaration = format!(".globl _{}\n_{}:", name, name);
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
    Function(Box<str>),
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
        }
    }
}
