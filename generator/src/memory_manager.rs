use std::{collections::HashMap, iter::zip, ops::Deref};

use errors::GeneratorErrors;

use crate::{
    assembly_flavour::FullInstruction,
    registers::{Registers, X86Registers},
};

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
    #[inline]
    pub fn reserve(&mut self, name: &str) -> Result<Location<R>, GeneratorErrors> {
        self.reserve_nth_free(name, 0)
    }

    #[inline]
    pub fn reserve_from_back(&mut self, name: &str) -> Result<Location<R>, GeneratorErrors> {
        self.reserve_nth_free(name, self.next_locations.len() - 1)
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
        if self.is_name_taken(name) || self.is_location_taken(&location) {
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
        match self.reservations.get(name) {
            Some(location @ Location::Register(register)) => {
                self.free_registers.push(register.clone());
                self.next_locations.insert(0, location.clone());
                self.reservations.remove(name);
            }
            Some(Location::Stack(_)) => {
                self.next_locations.insert(0, Location::Stack(0));
                self.reservations.remove(name);
            }
            Some(Location::Hardcoded(_)) | None => (),
        }
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

    #[inline]
    pub fn is_location_taken(&self, location: &Location<R>) -> bool {
        self.reservations.values().any(|l| l == location)
    }

    #[inline]
    pub fn is_name_taken(&self, name: &str) -> bool {
        self.reservations.contains_key(name)
    }
}

pub fn free_locations<R: Registers>(
    mm: &mut MemoryManager<R>,
    locations: &[Location<R>],
) -> Result<Vec<FullInstruction<R>>, GeneratorErrors> {
    let swap_temps = (0..locations.len()).map(|i| format!("@swap_temp_{}", i));

    for temp in swap_temps.clone() {
        mm.reserve(&temp)?;
    }

    let res = zip(locations.iter(), swap_temps.clone())
        .flat_map(|(location, temp)| {
            mm.get_name(location)
                .map(|s| s.to_owned())
                .map(|n| swap(mm, &n, &temp))
        })
        .flatten()
        .collect();

    for temp in swap_temps {
        mm.free(&temp);
    }

    Ok(res)
}

pub fn swap<R: Registers>(
    mm: &mut MemoryManager<R>,
    old_temp: &str,
    new_temp: &str,
) -> Result<FullInstruction<R>, GeneratorErrors> {
    let old_location = mm.get_location_or_err(old_temp)?.clone();
    let new_location = mm.get_location_or_err(new_temp)?.clone();

    mm.free(old_temp);
    mm.free(new_temp);

    mm.reserve_location(old_temp, new_location.clone())?;
    mm.reserve_location(new_temp, old_location.clone())?;

    Ok((
        crate::assembly_flavour::AssemblyInstruction::Swap,
        vec![old_location.into(), new_location.into()],
        format!("Swap {} and {}", old_temp, new_temp).into(),
    ))
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
                reservations: HashMap::from([("h_1".into(), Location::Hardcoded("h_1".into()))]),
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
                ("h_1".into(), Location::Hardcoded("h_1".into())),
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
                ("h_1".into(), Location::Hardcoded("h_1".into())),
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
    fn test_reserve_from_back() {
        let mut mm = MemoryManager::default();
        mm.reserve_from_back("a").unwrap();
        mm.reserve_from_back("b").unwrap();
        mm.reserve_from_back("c").unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Register(TestRegisters::R(1))),
                ("c".into(), Location::Register(TestRegisters::R(2)))
            ])
        );
        assert_eq!(mm.next_locations, Vec::from([Location::Stack(0)]));

        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.add_n_next_locations(3);
        mm.reserve_from_back("b").unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Stack(1))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([
                Location::Register(TestRegisters::R(1)),
                Location::Register(TestRegisters::R(2)),
                Location::Stack(0)
            ]),
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
                ("h_1".into(), Location::Hardcoded("h_1".into())),
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
                ("h_1".into(), Location::Hardcoded("h_1".into())),
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

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("c".into(), Location::Register(TestRegisters::R(2)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(1)), Location::Stack(0)])
        );

        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.free("h_1");

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(1)),])
        );
    }

    #[test]
    fn test_add_next_location() {
        let mut mm = MemoryManager::default();
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
        let mut mm = MemoryManager::default();
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
        let mut mm = MemoryManager::default();
        mm.reserve_location("a", Location::Register(TestRegisters::R(0)))
            .unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(1))])
        );
    }

    #[test]
    fn test_get_location() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();

        assert_eq!(
            mm.get_location("a"),
            Some(&Location::Register(TestRegisters::R(0)))
        );
        assert_eq!(mm.get_location("b"), None);
    }

    #[test]
    fn test_get_name() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();

        assert_eq!(
            mm.get_name(&Location::Register(TestRegisters::R(0))),
            Some("a")
        );
        assert_eq!(mm.get_name(&Location::Register(TestRegisters::R(1))), None);
    }

    #[test]
    fn test_is_location_taken() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve_location("b", Location::Stack(0)).unwrap();

        assert!(mm.is_location_taken(&Location::Register(TestRegisters::R(0))));
        assert!(!mm.is_location_taken(&Location::Register(TestRegisters::R(1))));
        assert!(mm.is_location_taken(&Location::Stack(0)));
    }

    #[test]
    fn test_is_name_taken() {
        let mut mm = MemoryManager::<TestRegisters>::default();
        mm.reserve("a").unwrap();
        mm.reserve_location("b", Location::Stack(0)).unwrap();

        assert!(mm.is_name_taken("a"));
        assert!(mm.is_name_taken("b"));
        assert!(!mm.is_name_taken("c"));
    }

    #[test]
    fn test_free_locations() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve("b").unwrap();
        mm.reserve("c").unwrap();
        mm.reserve("d").unwrap();

        assert_eq!(
            super::free_locations(
                &mut mm,
                &[
                    Location::Register(TestRegisters::R(0)),
                    Location::Register(TestRegisters::R(2)),
                    Location::Stack(0),
                    Location::Stack(4)
                ]
            )
            .unwrap(),
            [
                (
                    crate::assembly_flavour::AssemblyInstruction::Swap,
                    vec![
                        Location::Register(TestRegisters::R(0)).into(),
                        Location::Stack(1).into()
                    ],
                    "Swap a and @swap_temp_0".into()
                ),
                (
                    crate::assembly_flavour::AssemblyInstruction::Swap,
                    vec![
                        Location::Register(TestRegisters::R(2)).into(),
                        Location::Stack(2).into()
                    ],
                    "Swap c and @swap_temp_1".into()
                ),
                (
                    crate::assembly_flavour::AssemblyInstruction::Swap,
                    vec![Location::Stack(0).into(), Location::Stack(3).into()],
                    "Swap d and @swap_temp_2".into()
                )
            ]
        );

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Stack(1)),
                ("b".into(), Location::Register(TestRegisters::R(1))),
                ("c".into(), Location::Stack(2)),
                ("d".into(), Location::Stack(3))
            ])
        );
    }

    #[test]
    fn test_swap() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve("b").unwrap();

        assert_eq!(
            super::swap(&mut mm, "a", "b").unwrap(),
            (
                crate::assembly_flavour::AssemblyInstruction::Swap,
                vec![
                    Location::Register(TestRegisters::R(0)).into(),
                    Location::Register(TestRegisters::R(1)).into()
                ],
                "Swap a and b".into()
            )
        );

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(1))),
                ("b".into(), Location::Register(TestRegisters::R(0)))
            ])
        );
        assert_eq!(
            mm.next_locations,
            Vec::from([Location::Register(TestRegisters::R(2))])
        );
    }
}
