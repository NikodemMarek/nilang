pub trait ToAssembly {
    fn to_assembly(&self) -> Box<str>;
}
