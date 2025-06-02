use colored::Colorize;
use nilang_types::nodes::Type;

use crate::NilangErrorKind;

#[derive(Debug, Clone)]
pub enum TransformerErrors {
    TypeMismatch {
        expected: Type,
        found: Type,
    },
    FunctionNotFound(Box<str>),
    TypeNotFound(Box<str>),
    FunctionCallArgumentsMismatch {
        name: Box<str>,
        expected: usize,
        got: usize,
    },
    FieldsMismatch {
        expected: Box<[Box<str>]>,
        found: Box<[Box<str>]>,
    },
}

impl std::fmt::Display for TransformerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransformerErrors::TypeMismatch { expected, found } => {
                    format!(
                        "Type mismatch: expected `{}`, found `{}`",
                        match expected {
                            Type::Int => "int",
                            Type::Void => "void",
                            Type::Char => "char",
                            Type::String => "string",
                            Type::Object(name) => name,
                        },
                        match found {
                            Type::Int => "int",
                            Type::Void => "void",
                            Type::Char => "char",
                            Type::String => "string",
                            Type::Object(name) => name,
                        }
                    )
                    .as_str()
                    .red()
                }
                TransformerErrors::FunctionNotFound(name) => {
                    format!("Function not found: `{}`", name).as_str().red()
                }
                TransformerErrors::TypeNotFound(name) => {
                    format!("Type not found: `{}`", name).as_str().red()
                }
                TransformerErrors::FunctionCallArgumentsMismatch {
                    name,
                    expected,
                    got,
                } => {
                    format!(
                        "Function call arguments mismatch: `{}` expected `{}`, got `{}`",
                        name, expected, got
                    )
                    .as_str()
                    .red()
                }
                TransformerErrors::FieldsMismatch { expected, found } => {
                    format!(
                        "Fields mismatch: expected `{}`, found `{}`",
                        expected.join(", "),
                        found.join(", ")
                    )
                    .as_str()
                    .red()
                }
            }
        )
    }
}

impl From<TransformerErrors> for NilangErrorKind {
    fn from(error: TransformerErrors) -> Self {
        NilangErrorKind::TransformerError(error)
    }
}

impl std::error::Error for TransformerErrors {}
