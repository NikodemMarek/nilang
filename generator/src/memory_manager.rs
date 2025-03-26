use std::collections::HashMap;

use crate::registers::Registers;

#[derive(Debug, Clone, PartialEq)]
pub enum Location {
    Register(Registers),
    Stack(usize),
}

#[derive(Debug)]
pub struct MemoryManager {
    // FIXME: do tests in some other way, so it does not require publics here
    pub stack_position: usize,
    pub free_registers: Vec<Registers>,

    pub next_locations: Vec<Location>,
    pub reservations: HashMap<Box<str>, Location>,
}

impl MemoryManager {
    pub fn reserve(&mut self, name: &str) -> Location {
        let location = self.next_locations.pop().unwrap();
        self.reserve_location(name, location.clone()).unwrap();
        location
    }

    pub fn reserve_from_back(&mut self, name: &str) -> Location {
        let location = self.next_locations.remove(0);
        self.reserve_location(name, location.clone()).unwrap();
        location
    }

    pub fn reserve_location(&mut self, name: &str, location: Location) -> Result<(), ()> {
        if self.is_taken(&location) {
            return Err(());
        }

        if let Location::Register(ref register) = location {
            self.free_registers.retain(|r| r != register);
        }
        self.reservations.insert(name.into(), location.clone());

        if self.next_locations.contains(&location) {
            self.next_locations.remove(0);
        }
        if self.next_locations.is_empty() {
            self.add_next_location();
        }

        Ok(())
    }

    pub fn free(&mut self, name: &str) {
        let location = self.reservations.remove(name).unwrap();
        if let Location::Register(ref register) = location {
            self.free_registers.push(register.clone());
        }
        self.next_locations.insert(0, location);
    }

    fn add_next_location(&mut self) {
        self.next_locations
            .push(if let Some(register) = self.free_registers.last() {
                Location::Register(register.clone())
            } else {
                self.stack_position += 1;
                Location::Stack(self.stack_position)
            });
    }

    pub fn add_n_next_locations(&mut self, n: usize) {
        let mut i = 0;
        let mut n = n;
        while i < n {
            self.next_locations.push(
                if let Some(register) = self.free_registers.get(self.free_registers.len() - i - 1) {
                    if self
                        .next_locations
                        .contains(&Location::Register(register.clone()))
                    {
                        i += 1;
                        n += 1;
                        continue;
                    } else {
                        Location::Register(register.clone())
                    }
                } else {
                    self.stack_position += 1;
                    Location::Stack(self.stack_position)
                },
            );
            i += 1;
        }
    }

    pub fn get_location(&self, name: &str) -> Option<&Location> {
        self.reservations.get(name)
    }

    pub fn get_name(&self, location: &Location) -> Option<&Box<str>> {
        self.reservations.iter().find_map(
            |(name, loc)| {
                if loc == location {
                    Some(name)
                } else {
                    None
                }
            },
        )
    }

    pub fn is_taken(&self, location: &Location) -> bool {
        self.reservations.values().any(|l| l == location)
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            stack_position: 0,
            free_registers: vec![
                Registers::R15,
                Registers::R14,
                Registers::R13,
                Registers::R12,
                Registers::R11,
                Registers::R10,
                Registers::R9,
                Registers::R8,
                Registers::Rdi,
                Registers::Rsi,
                Registers::Rdx,
                Registers::Rcx,
                Registers::Rbx,
                Registers::Rax,
            ],
            next_locations: Vec::from([Location::Register(Registers::Rax)]),
            reservations: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{memory_manager::Location, registers::Registers};

    #[test]
    fn test_reserve() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a");
        mm.reserve("b");
        mm.reserve("c");

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(Registers::Rax)),
                ("b".into(), Location::Register(Registers::Rbx)),
                ("c".into(), Location::Register(Registers::Rcx))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(Registers::Rdx)])
        );
    }

    #[test]
    fn test_free() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a");
        mm.reserve("b");
        mm.reserve("c");

        mm.free("b");

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("a".into(), Location::Register(Registers::Rax)),
                ("c".into(), Location::Register(Registers::Rcx))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(Registers::Rbx),
                Location::Register(Registers::Rdx)
            ])
        );
    }

    #[test]
    fn test_reserve_location() {
        let mut mm = super::MemoryManager::default();
        mm.reserve_location("a", Location::Register(Registers::Rdi))
            .unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([("a".into(), Location::Register(Registers::Rdi))])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(Registers::Rax)])
        );
    }

    #[test]
    fn test_get_location() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a");

        assert_eq!(
            mm.get_location("a"),
            Some(&Location::Register(Registers::Rax))
        );
        assert_eq!(mm.get_location("b"), None);
    }

    #[test]
    fn test_get_name() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a");

        assert_eq!(
            mm.get_name(&Location::Register(Registers::Rax)),
            Some(&"a".into())
        );
        assert_eq!(mm.get_name(&Location::Register(Registers::Rbx)), None);
    }

    #[test]
    fn test_is_taken() {
        let mut mm = super::MemoryManager::default();
        mm.reserve("a");

        assert!(mm.is_taken(&Location::Register(Registers::Rax)));
        assert!(!mm.is_taken(&Location::Register(Registers::Rbx)));
    }
}
