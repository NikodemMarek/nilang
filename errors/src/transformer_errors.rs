use colored::Colorize;
use nilang_types::nodes::Type;

#[derive(Debug, Clone)]
pub enum TransformerErrors {
    TemporaryNotFound {
        name: Box<str>,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
    },
    FunctionNotFound {
        name: Box<str>,
    },
    TypeNotFound {
        name: Box<str>,
    },
    FunctionCallArgumentsMismatch {
        name: Box<str>,
        expected: usize,
        got: usize,
    },
}

impl std::fmt::Display for TransformerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransformerErrors::TemporaryNotFound { name } => {
                    format!("Temporary not found: `{}`", name).as_str().red()
                }
                TransformerErrors::TypeMismatch { expected, found } => {
                    format!(
                        "Type mismatch: expected `{}`, found `{}`",
                        match expected {
                            Type::Int => "int",
                            Type::Void => "void",
                            Type::Char => "char",
                            Type::Object(name) => name,
                        },
                        match found {
                            Type::Int => "int",
                            Type::Void => "void",
                            Type::Char => "char",
                            Type::Object(name) => name,
                        }
                    )
                    .as_str()
                    .red()
                }
                TransformerErrors::FunctionNotFound { name } => {
                    format!("Function not found: `{}`", name).as_str().red()
                }
                TransformerErrors::TypeNotFound { name } => {
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
            }
        )
    }
}

impl std::error::Error for TransformerErrors {}
