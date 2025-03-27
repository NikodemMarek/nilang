use std::{collections::HashMap, ops::Deref};

use errors::GeneratorErrors;

use crate::registers::{Registers, X86Registers};

#[derive(Debug, Clone, PartialEq)]
pub enum Location<R: Registers> {
    Register(R),
    Stack(usize),
    Hardcoded(Box<str>),
}

#[derive(Debug)]
pub struct MemoryManager<R: Registers> {
    // FIXME: do tests in some other way, so it does not require publics here
    pub stack_position: usize,
    pub free_registers: Vec<R>,

    pub next_locations: Vec<Location<R>>,
    pub reservations: HashMap<Box<str>, Location<R>>,
}

impl<R: Registers> MemoryManager<R> {
    pub fn reserve(&mut self, name: &str) -> Result<Location<R>, GeneratorErrors> {
        let location = self.next_locations.remove(0);
        self.reserve_location(name, location.clone())?;
        Ok(location)
    }

    pub fn reserve_nth_free(
        &mut self,
        name: &str,
        n: usize,
    ) -> Result<Location<R>, GeneratorErrors> {
        self.add_n_next_locations(n.saturating_sub(self.next_locations.len() - 1));
        let location = self.next_locations.remove(n);
        self.reserve_location(name, location.clone())?;
        Ok(location)
    }

    pub fn reserve_location(
        &mut self,
        name: &str,
        location: Location<R>,
    ) -> Result<(), GeneratorErrors> {
        if self.is_taken(&location) {
            return Err(GeneratorErrors::VariableAlreadyExists { name: name.into() });
        }

        if let Location::Register(ref register) = location {
            self.free_registers.retain(|r| r != register);
        }
        self.reservations.insert(name.into(), location.clone());

        if self.next_locations.contains(&location) {
            self.next_locations.retain(|l| l != &location);
        }
        if self.next_locations.is_empty() {
            self.add_next_location();
        }

        Ok(())
    }

    pub fn free(&mut self, name: &str) {
        let Some(location) = self.reservations.remove(name) else {
            return;
        };

        if let Location::Register(ref register) = location {
            self.free_registers.push(register.clone());
        }
        self.next_locations.insert(0, location);
    }

    fn add_next_location(&mut self) {
        self.next_locations
            .push(if let Some(register) = self.free_registers.pop() {
                Location::Register(register.clone())
            } else {
                self.stack_position += 1;
                Location::Stack(self.stack_position - 1)
            });
    }

    pub fn add_n_next_locations(&mut self, n: usize) {
        for _ in 0..n {
            self.add_next_location();
        }
    }

    fn get_location(&self, name: &str) -> Option<&Location<R>> {
        self.reservations.get(name)
    }

    pub fn get_location_or_err(&self, name: &str) -> Result<&Location<R>, GeneratorErrors> {
        self.get_location(name)
            .ok_or_else(|| GeneratorErrors::VariableNotDefined { name: name.into() })
    }

    pub fn get_name(&self, location: &Location<R>) -> Option<&str> {
        self.reservations.iter().find_map(|(name, loc)| {
            if loc == location {
                Some(name.deref())
            } else {
                None
            }
        })
    }

    pub fn is_taken(&self, location: &Location<R>) -> bool {
        self.reservations.values().any(|l| l == location)
    }
}

impl Default for MemoryManager<X86Registers> {
    fn default() -> Self {
        Self {
            stack_position: 0,
            free_registers: vec![
                X86Registers::R15,
                X86Registers::R14,
                X86Registers::R13,
                X86Registers::R12,
                X86Registers::R11,
                X86Registers::R10,
                X86Registers::R9,
                X86Registers::R8,
                X86Registers::Rdi,
                X86Registers::Rsi,
                X86Registers::Rdx,
                X86Registers::Rcx,
                X86Registers::Rbx,
            ],
            next_locations: Vec::from([Location::Register(X86Registers::Rax)]),
            reservations: HashMap::from([
                (
                    "printi_format".into(),
                    Location::Hardcoded("printi_format".into()),
                ),
                (
                    "printc_format".into(),
                    Location::Hardcoded("printc_format".into()),
                ),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        memory_manager::{Location, MemoryManager},
        registers::tests::TestRegisters,
    };

    impl Default for MemoryManager<TestRegisters> {
        fn default() -> Self {
            Self {
                stack_position: 0,
                free_registers: vec![TestRegisters::R(2), TestRegisters::R(1)],
                next_locations: Vec::from([Location::Register(TestRegisters::R(0))]),
                reservations: HashMap::new(),
            }
        }
    }

    #[test]
    fn test_reserve() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve("b").unwrap();
        mm.reserve("c").unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Register(TestRegisters::R(1))),
                ("c".into(), Location::Register(TestRegisters::R(2)))
            ])
        );
        assert_eq!(mm.next_locations, Vec::from([Location::Stack(0)]));

        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.add_n_next_locations(4);
        mm.reserve("b").unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Register(TestRegisters::R(1)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(2)),
                Location::Stack(0),
                Location::Stack(1),
                Location::Stack(2),
            ])
        );
    }

    #[test]
    fn test_reserve_nth_free() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve_nth_free("b", 3).unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Stack(1)),
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(1)),
                Location::Register(TestRegisters::R(2)),
                Location::Stack(0)
            ])
        );

        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve_nth_free("b", 2).unwrap();
        mm.reserve_nth_free("c", 1).unwrap();
        mm.reserve_nth_free("d", 3).unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Stack(0)),
                ("c".into(), Location::Register(TestRegisters::R(2))),
                ("d".into(), Location::Stack(3)),
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(1)),
                Location::Stack(1),
                Location::Stack(2),
            ])
        );
    }

    #[test]
    fn test_free() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve("b").unwrap();
        mm.reserve("c").unwrap();

        mm.free("b");

        dbg!(&mm);

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("c".into(), Location::Register(TestRegisters::R(2)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(1)), Location::Stack(0)])
        );
    }

    #[test]
    fn test_add_next_location() {
        let mut mm = super::MemoryManager::default();
        mm.add_next_location();
        mm.add_next_location();

        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(0)),
                Location::Register(TestRegisters::R(1)),
                Location::Register(TestRegisters::R(2))
            ])
        );
    }

    #[test]
    fn test_add_n_next_locations() {
        let mut mm = super::MemoryManager::default();
        mm.add_n_next_locations(3);
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(0)),
                Location::Register(TestRegisters::R(1)),
                Location::Register(TestRegisters::R(2)),
                Location::Stack(0)
            ])
        );
    }

    #[test]
    fn test_reserve_location() {
        let mut mm = super::MemoryManager::default();
        mm.reserve_location("a", Location::Register(TestRegisters::R(0)))
            .unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([("a".into(), Location::Register(TestRegisters::R(0)))])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(1))])
        );
    }

    #[test]
    fn test_get_location() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a").unwrap();

        assert_eq!(
            mm.get_location("a"),
            Some(&Location::Register(TestRegisters::R(0)))
        );
        assert_eq!(mm.get_location("b"), None);
    }

    #[test]
    fn test_get_name() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a").unwrap();

        assert_eq!(
            mm.get_name(&Location::Register(TestRegisters::R(0))),
            Some("a")
        );
        assert_eq!(mm.get_name(&Location::Register(TestRegisters::R(1))), None);
    }

    #[test]
    fn test_is_taken() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a").unwrap();

        assert!(mm.is_taken(&Location::Register(TestRegisters::R(0))));
        assert!(!mm.is_taken(&Location::Register(TestRegisters::R(1))));
    }
}
