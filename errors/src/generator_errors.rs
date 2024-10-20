use colored::Colorize;
use nilang_types::nodes::Node;

#[derive(Debug, Clone)]
pub enum GeneratorErrors {
    InvalidNode { node: Node },
    VariableAlreadyExists { name: String },
    VariableDoesNotExist { name: String },
}

impl std::fmt::Display for GeneratorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneratorErrors::InvalidNode { node } => {
                    format!("Invalid node: {:?}", node).as_str().red()
                }
                GeneratorErrors::VariableAlreadyExists { name } => {
                    format!("Variable `{}` already exists", name).as_str().red()
                }
                GeneratorErrors::VariableDoesNotExist { name } => {
                    format!("Variable `{}` does not exist", name).as_str().red()
                }
            }
        )
    }
}

impl std::error::Error for GeneratorErrors {}
