use colored::Colorize;

use crate::NilangErrorKind;

#[derive(Debug, Clone)]
pub enum GeneratorErrors {
    VariableAlreadyExists {
        name: Box<str>,
    },
    VariableNotDefined {
        name: Box<str>,
    },
    StructureNotDefined {
        name: Box<str>,
    },
    FieldNotDefined {
        name: Box<str>,
    },
    InvalidType {
        expected: Box<str>,
        received: Box<str>,
    },
}

impl std::fmt::Display for GeneratorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneratorErrors::VariableAlreadyExists { name } => {
                    format!("Variable `{}` already exists", name).as_str().red()
                }
                GeneratorErrors::VariableNotDefined { name } => {
                    format!("Variable `{}` does not exist", name).as_str().red()
                }
                GeneratorErrors::StructureNotDefined { name } => {
                    format!("Structure `{}` not defined", name).as_str().red()
                }
                GeneratorErrors::FieldNotDefined { name } => {
                    format!("Field `{}` not defined", name).as_str().red()
                }
                GeneratorErrors::InvalidType { expected, received } => {
                    format!(
                        "Invalid type: expected `{}`, received `{}`",
                        expected, received
                    )
                    .as_str()
                    .red()
                }
            }
        )
    }
}

impl From<GeneratorErrors> for NilangErrorKind {
    fn from(value: GeneratorErrors) -> Self {
        NilangErrorKind::GeneratorError(value)
    }
}

impl std::error::Error for GeneratorErrors {}
