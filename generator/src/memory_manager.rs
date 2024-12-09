use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register {
    pub fn value(&self) -> &'static str {
        match self {
            Register::RAX => "%rax",
            Register::RBX => "%rbx",
            Register::RCX => "%rcx",
            Register::RDX => "%rdx",
            Register::RSI => "%rsi",
            Register::RDI => "%rdi",
            Register::RBP => "%rbp",
            Register::RSP => "%rsp",
            Register::R8 => "%r8",
            Register::R9 => "%r9",
            Register::R10 => "%r10",
            Register::R11 => "%r11",
            Register::R12 => "%r12",
            Register::R13 => "%r13",
            Register::R14 => "%r14",
            Register::R15 => "%r15",
        }
    }
}

pub struct MemoryManager {
    registers: Vec<Register>,
    reservations: HashMap<Box<str>, Register>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            registers: [
                Register::RAX,
                Register::RBX,
                Register::RCX,
                Register::RDX,
                Register::RSI,
                Register::RDI,
                Register::RBP,
                Register::RSP,
                Register::R8,
                Register::R9,
                Register::R10,
                Register::R11,
                Register::R12,
                Register::R13,
                Register::R14,
                Register::R15,
            ]
            .into(),
            reservations: HashMap::new(),
        }
    }
}

impl MemoryManager {
    #[inline]
    pub fn reserve(&mut self, name: &str) -> &'static str {
        let register = match self.registers.pop() {
            Some(register) => register,
            None => unreachable!(),
        };

        self.reservations.insert(name.into(), register);

        register.value()
    }
}
