pub trait Registers: Clone + PartialEq {
    fn nth(n: usize) -> Option<Self>;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, PartialEq)]
pub enum X86Registers {
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

impl Registers for X86Registers {
    fn nth(n: usize) -> Option<Self> {
        match n {
            0 => Some(X86Registers::Rax),
            1 => Some(X86Registers::Rbx),
            2 => Some(X86Registers::Rcx),
            3 => Some(X86Registers::Rdx),
            4 => Some(X86Registers::Rsi),
            5 => Some(X86Registers::Rdi),
            6 => Some(X86Registers::Rbp),
            7 => Some(X86Registers::Rsp),
            8 => Some(X86Registers::R8),
            9 => Some(X86Registers::R9),
            10 => Some(X86Registers::R10),
            11 => Some(X86Registers::R11),
            12 => Some(X86Registers::R12),
            13 => Some(X86Registers::R13),
            14 => Some(X86Registers::R14),
            15 => Some(X86Registers::R15),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            X86Registers::Rax => "rax",
            X86Registers::Rbx => "rbx",
            X86Registers::Rcx => "rcx",
            X86Registers::Rdx => "rdx",
            X86Registers::Rsi => "rsi",
            X86Registers::Rdi => "rdi",
            X86Registers::Rbp => "rbp",
            X86Registers::Rsp => "rsp",
            X86Registers::R8 => "r8",
            X86Registers::R9 => "r9",
            X86Registers::R10 => "r10",
            X86Registers::R11 => "r11",
            X86Registers::R12 => "r12",
            X86Registers::R13 => "r13",
            X86Registers::R14 => "r14",
            X86Registers::R15 => "r15",
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::Registers;

    #[derive(Debug, Clone, PartialEq)]
    pub enum TestRegisters {
        R(usize),
    }

    impl Registers for TestRegisters {
        fn nth(n: usize) -> Option<Self> {
            if n < 4 {
                Some(TestRegisters::R(n))
            } else {
                None
            }
        }

        fn name(&self) -> &'static str {
            "test_register"
        }
    }
}
