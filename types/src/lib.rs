use std::ops::Deref;

pub mod instructions;
pub mod nodes;
pub mod tokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(pub usize, pub usize, pub usize, pub usize);
impl Location {
    pub fn at(line: usize, char: usize) -> Self {
        Self(line, char, line, char)
    }
    pub fn range(line_from: usize, char_from: usize, line_to: usize, char_to: usize) -> Self {
        Self(line_from, char_from, line_to, char_to)
    }
    pub fn between(start: &Self, end: &Self) -> Self {
        Self(start.0, start.1, end.2, end.3)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Localizable<T: Sized + Clone> {
    pub location: Location,
    pub object: T,
}

impl<T: Sized + Clone> Localizable<T> {
    pub fn new(location: Location, object: T) -> Self {
        Self { location, object }
    }
}

#[cfg(test)]
impl<T: Sized + Clone> Localizable<T> {
    pub fn irrelevant(object: T) -> Self {
        Self {
            location: Location::at(0, 0),
            object,
        }
    }
}

impl<T: Sized + Clone> Deref for Localizable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
