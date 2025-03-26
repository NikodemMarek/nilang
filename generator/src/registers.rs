#[derive(Debug, Clone, PartialEq)]
pub enum Registers {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Registers {
    pub fn nth(n: usize) -> Option<Self> {
        match n {
            0 => Some(Registers::Rax),
            1 => Some(Registers::Rbx),
            2 => Some(Registers::Rcx),
            3 => Some(Registers::Rdx),
            4 => Some(Registers::Rsi),
            5 => Some(Registers::Rdi),
            6 => Some(Registers::Rbp),
            7 => Some(Registers::Rsp),
            8 => Some(Registers::R8),
            9 => Some(Registers::R9),
            10 => Some(Registers::R10),
            11 => Some(Registers::R11),
            12 => Some(Registers::R12),
            13 => Some(Registers::R13),
            14 => Some(Registers::R14),
            15 => Some(Registers::R15),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Registers::Rax => "rax",
            Registers::Rbx => "rbx",
            Registers::Rcx => "rcx",
            Registers::Rdx => "rdx",
            Registers::Rsi => "rsi",
            Registers::Rdi => "rdi",
            Registers::Rbp => "rbp",
            Registers::Rsp => "rsp",
            Registers::R8 => "r8",
            Registers::R9 => "r9",
            Registers::R10 => "r10",
            Registers::R11 => "r11",
            Registers::R12 => "r12",
            Registers::R13 => "r13",
            Registers::R14 => "r14",
            Registers::R15 => "r15",
        }
    }
}
