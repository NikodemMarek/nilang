use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
    ops::Deref,
};

use errors::GeneratorErrors;

use crate::{assembly_flavour::FullInstruction, registers::Registers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location<R: Registers> {
    Register(R),
    Stack(usize),
    Hardcoded(Box<str>),
}

impl<R: Registers> PartialOrd for Location<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registers> Ord for Location<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Location::Register(a), Location::Register(b)) => a.cmp(b),
            (Location::Stack(a), Location::Stack(b)) => a.cmp(b).reverse(),
            (Location::Hardcoded(_), Location::Hardcoded(_)) => std::cmp::Ordering::Equal,
            (Location::Register(_), _) => std::cmp::Ordering::Greater,
            (Location::Stack(_), Location::Register(_)) => std::cmp::Ordering::Less,
            (Location::Stack(_), Location::Hardcoded(_)) => std::cmp::Ordering::Greater,
            (Location::Hardcoded(_), _) => std::cmp::Ordering::Less,
        }
    }
}

#[derive(Debug)]
pub struct MemoryManager<R: Registers> {
    stack_position: usize,
    next_locations: BinaryHeap<Location<R>>,

    reservations: HashMap<Box<str>, Location<R>>,
}

impl<R: Registers> Default for MemoryManager<R> {
    fn default() -> Self {
        Self {
            stack_position: 0,
            next_locations: BinaryHeap::from(
                R::all()
                    .iter()
                    .map(|r| Location::Register(*r))
                    .collect::<Vec<Location<R>>>(),
            ),
            reservations: HashMap::new(),
        }
    }
}

impl<R: Registers> MemoryManager<R> {
    pub fn new(builtins: &[Box<str>]) -> Self {
        Self {
            reservations: builtins
                .iter()
                .map(|l| (l.clone(), Location::Hardcoded(l.clone())))
                .collect(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn reserve(&mut self, name: &str) -> Result<Location<R>, GeneratorErrors> {
        self.reserve_nth_free(name, 1)
    }

    pub fn reserve_nth_free(
        &mut self,
        name: &str,
        n: usize,
    ) -> Result<Location<R>, GeneratorErrors> {
        self.ensure_n_next_locations(n);

        let mut retained = (0..n.saturating_sub(1))
            .map(|_| self.next_locations.pop().unwrap())
            .collect();

        let location = self.next_locations.pop().unwrap();
        self.reserve_location(name, location.clone())?;

        self.next_locations.append(&mut retained);
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

        self.reservations.insert(name.into(), location.clone());

        self.next_locations.retain(|l| l != &location);

        Ok(())
    }

    pub fn free(&mut self, name: &str) {
        match self.reservations.get(name) {
            Some(Location::Hardcoded(_)) | None => (),
            Some(location) => {
                self.next_locations.push(location.to_owned());
                self.reservations.remove(name);
            }
        }
    }

    fn add_next_location(&mut self) {
        self.stack_position += 1;
        self.next_locations
            .push(Location::Stack(self.stack_position - 1));
    }

    pub fn ensure_n_next_locations(&mut self, n: usize) {
        for _ in 0..n.saturating_sub(self.next_locations.len()) {
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

#[cfg(test)]
mod tests {
    use std::{
        collections::{BinaryHeap, HashMap},
        usize,
    };

    use crate::{
        memory_manager::{Location, MemoryManager},
        registers::tests::TestRegisters,
    };

    fn test_builtin_variables() -> [Box<str>; 1] {
        ["h_1".into()]
    }

    #[test]
    fn test_location_ord() {
        assert!(Location::Register(TestRegisters::R(0)) > Location::Register(TestRegisters::R(1)));
        assert!(Location::Register(TestRegisters::R(1)) > Location::Register(TestRegisters::R(2)));

        assert!(Location::Register(TestRegisters::R(0)) > Location::Stack(0));
        assert!(Location::Register(TestRegisters::R(2)) > Location::Stack(0));
        assert!(Location::Register(TestRegisters::R(0)) > Location::Stack(usize::MAX));
        assert!(Location::Register(TestRegisters::R(2)) > Location::Stack(usize::MAX));

        assert!(Location::<TestRegisters>::Stack(0) > Location::Stack(1));
        assert!(Location::<TestRegisters>::Stack(0) > Location::Stack(usize::MAX));
        assert!(Location::<TestRegisters>::Stack(usize::MAX - 1) > Location::Stack(usize::MAX));

        assert!(Location::Register(TestRegisters::R(0)) > Location::Hardcoded("a".into()));
        assert!(Location::Register(TestRegisters::R(2)) > Location::Hardcoded("a".into()));
        assert!(Location::<TestRegisters>::Stack(0) > Location::Hardcoded("a".into()));
        assert!(Location::<TestRegisters>::Stack(usize::MAX) > Location::Hardcoded("a".into()));

        let mut registers = BinaryHeap::from([
            Location::Register(TestRegisters::R(2)),
            Location::Stack(0),
            Location::Register(TestRegisters::R(1)),
            Location::Hardcoded("a".into()),
            Location::Stack(1),
        ]);

        assert_eq!(
            registers.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            registers.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(registers.pop(), Some(Location::Stack(0)));
        assert_eq!(registers.pop(), Some(Location::Stack(1)));
        assert_eq!(registers.pop(), Some(Location::Hardcoded("a".into())));
        assert_eq!(registers.pop(), None);
    }

    #[test]
    fn test_reserve() {
        let mut mm = MemoryManager::new(&test_builtin_variables());
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

        assert_eq!(mm.next_locations.pop(), None);

        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.ensure_n_next_locations(5);
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
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(0)));
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(1)));
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(2)));
        assert_eq!(mm.next_locations.pop(), None);
    }

    #[test]
    fn test_reserve_nth_free() {
        let mut mm = MemoryManager::default();
        mm.reserve("a").unwrap();
        mm.reserve_nth_free("b", 4).unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Stack(1)),
            ])
        );

        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(0)));

        let mut mm = MemoryManager::default();
        mm.reserve_nth_free("a", 1).unwrap();
        mm.reserve_nth_free("b", 2).unwrap();
        mm.reserve_nth_free("c", 1).unwrap();
        mm.reserve_nth_free("d", 3).unwrap();

        assert_eq!(
            mm.reservations,
            HashMap::from([
                ("h_1".into(), Location::Hardcoded("h_1".into())),
                ("a".into(), Location::Register(TestRegisters::R(0))),
                ("b".into(), Location::Register(TestRegisters::R(2))),
                ("c".into(), Location::Register(TestRegisters::R(1))),
                ("d".into(), Location::Stack(2)),
            ])
        );

        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(0)));
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(1)));
        assert_eq!(mm.next_locations.pop(), None);
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
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(mm.next_locations.pop(), None);

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
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), None);
    }

    #[test]
    fn test_add_next_location() {
        let mut mm = MemoryManager::default();
        mm.add_next_location();
        mm.add_next_location();

        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(0)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(0)));
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(1)));
        assert_eq!(mm.next_locations.pop(), None);
    }

    #[test]
    fn test_ensure_n_next_locations() {
        let mut mm = MemoryManager::default();
        mm.ensure_n_next_locations(4);

        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(0)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), Some(Location::Stack(0)));
        assert_eq!(mm.next_locations.pop(), None);
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
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(1)))
        );
        assert_eq!(
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), None);
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
            mm.next_locations.pop(),
            Some(Location::Register(TestRegisters::R(2)))
        );
        assert_eq!(mm.next_locations.pop(), None);
    }
}
