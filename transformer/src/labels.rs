use std::cell::Cell;

#[derive(Debug, Default)]
pub struct Labels(Cell<usize>);

impl Labels {
    pub fn create(&self) -> Box<str> {
        let current = self.0.get();
        let name = <Box<str>>::from(format!("label_{}", current));
        self.0.set(current + 1);
        name
    }
}
