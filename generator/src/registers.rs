pub trait Registers:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + std::fmt::Display + std::fmt::Debug
{
    const COUNT: usize;
    fn all() -> Box<[Self]>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    const COUNT: usize = 16;
    fn all() -> Box<[Self]> {
        Box::new([
            X86Registers::Rax,
            X86Registers::Rbx,
            X86Registers::Rcx,
            X86Registers::Rdx,
            X86Registers::Rsi,
            X86Registers::Rdi,
            X86Registers::R8,
            X86Registers::R9,
            X86Registers::R10,
            X86Registers::R11,
            X86Registers::R12,
            X86Registers::R13,
            X86Registers::R14,
            X86Registers::R15,
        ])
    }
}

impl std::fmt::Display for X86Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
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
        )
    }
}

impl PartialOrd for X86Registers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for X86Registers {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (X86Registers::Rax, X86Registers::Rax) => Ordering::Equal,
            (X86Registers::Rax, _) => Ordering::Greater,
            (_, X86Registers::Rax) => Ordering::Less,

            (X86Registers::Rbx, X86Registers::Rbx) => Ordering::Equal,
            (X86Registers::Rbx, _) => Ordering::Greater,
            (_, X86Registers::Rbx) => Ordering::Less,

            (X86Registers::Rcx, X86Registers::Rcx) => Ordering::Equal,
            (X86Registers::Rcx, _) => Ordering::Greater,
            (_, X86Registers::Rcx) => Ordering::Less,

            (X86Registers::Rdx, X86Registers::Rdx) => Ordering::Equal,
            (X86Registers::Rdx, _) => Ordering::Greater,
            (_, X86Registers::Rdx) => Ordering::Less,

            (X86Registers::Rsi, X86Registers::Rsi) => Ordering::Equal,
            (X86Registers::Rsi, _) => Ordering::Greater,
            (_, X86Registers::Rsi) => Ordering::Less,

            (X86Registers::Rdi, X86Registers::Rdi) => Ordering::Equal,
            (X86Registers::Rdi, _) => Ordering::Greater,
            (_, X86Registers::Rdi) => Ordering::Less,

            (X86Registers::Rbp, X86Registers::Rbp) => Ordering::Equal,
            (X86Registers::Rbp, _) => Ordering::Greater,
            (_, X86Registers::Rbp) => Ordering::Less,

            (X86Registers::Rsp, X86Registers::Rsp) => Ordering::Equal,
            (X86Registers::Rsp, _) => Ordering::Greater,
            (_, X86Registers::Rsp) => Ordering::Less,

            (X86Registers::R8, X86Registers::R8) => Ordering::Equal,
            (X86Registers::R8, _) => Ordering::Greater,
            (_, X86Registers::R8) => Ordering::Less,

            (X86Registers::R9, X86Registers::R9) => Ordering::Equal,
            (X86Registers::R9, _) => Ordering::Greater,
            (_, X86Registers::R9) => Ordering::Less,

            (X86Registers::R10, X86Registers::R10) => Ordering::Equal,
            (X86Registers::R10, _) => Ordering::Greater,
            (_, X86Registers::R10) => Ordering::Less,

            (X86Registers::R11, X86Registers::R11) => Ordering::Equal,
            (X86Registers::R11, _) => Ordering::Greater,
            (_, X86Registers::R11) => Ordering::Less,

            (X86Registers::R12, X86Registers::R12) => Ordering::Equal,
            (X86Registers::R12, _) => Ordering::Greater,
            (_, X86Registers::R12) => Ordering::Less,

            (X86Registers::R13, X86Registers::R13) => Ordering::Equal,
            (X86Registers::R13, _) => Ordering::Greater,
            (_, X86Registers::R13) => Ordering::Less,

            (X86Registers::R14, X86Registers::R14) => Ordering::Equal,
            (X86Registers::R14, _) => Ordering::Greater,
            (_, X86Registers::R14) => Ordering::Less,

            (X86Registers::R15, X86Registers::R15) => Ordering::Equal,
            // (X86Registers::R15, _) => Ordering::Greater,
            // (_, X86Registers::R15) => Ordering::Less,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::BinaryHeap;

    use super::Registers;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum TestRegisters {
        R(usize),
    }

    impl PartialOrd for TestRegisters {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for TestRegisters {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self, other) {
                (TestRegisters::R(a), TestRegisters::R(b)) => a.cmp(b).reverse(),
            }
        }
    }

    impl Registers for TestRegisters {
        const COUNT: usize = 4;
        fn all() -> Box<[Self]> {
            Box::new([
                TestRegisters::R(0),
                TestRegisters::R(1),
                TestRegisters::R(2),
            ])
        }
    }

    impl std::fmt::Display for TestRegisters {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "r{}",
                match self {
                    TestRegisters::R(r) => r.to_string(),
                }
            )
        }
    }

    #[test]
    fn test_registers_ord() {
        assert!(TestRegisters::R(0) > TestRegisters::R(1));
        assert!(TestRegisters::R(1) > TestRegisters::R(2));

        let mut registers = BinaryHeap::from([
            TestRegisters::R(1),
            TestRegisters::R(0),
            TestRegisters::R(2),
        ]);

        assert_eq!(registers.pop(), Some(TestRegisters::R(0)));
        assert_eq!(registers.pop(), Some(TestRegisters::R(1)));
        assert_eq!(registers.pop(), Some(TestRegisters::R(2)));
        assert_eq!(registers.pop(), None);
    }
}
