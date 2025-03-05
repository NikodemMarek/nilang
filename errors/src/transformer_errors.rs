use colored::Colorize;

#[derive(Debug, Clone)]
pub enum TransformerErrors {
    TemporaryNotFound {
        name: Box<str>,
    },
    InvalidType {
        expected: Box<str>,
        received: Box<str>,
    },
    FunctionNotFound {
        name: Box<str>,
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
                TransformerErrors::InvalidType { expected, received } => {
                    format!(
                        "Invalid type: expected `{}`, received `{}`",
                        expected, received
                    )
                    .as_str()
                    .red()
                }
                TransformerErrors::FunctionNotFound { name } => {
                    format!("Function not found: `{}`", name).as_str().red()
                }
            }
        )
    }
}

impl std::error::Error for TransformerErrors {}
