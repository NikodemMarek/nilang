pub mod expressions;
pub mod statements;

use std::fmt::Debug;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    #[default]
    Void,
    Bool,
    Int,
    Char,
    String,
    Object(Box<str>),
}
