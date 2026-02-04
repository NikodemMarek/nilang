pub mod expressions;
pub mod statements;

use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Void,
    Bool,
    Int,
    Char,
    String,
    Object(Box<str>),
}
