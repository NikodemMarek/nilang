use colored::Colorize;
use nilang_types::nodes::Node;

#[derive(Debug, Clone)]
pub enum GeneratorErrors {
    InvalidNode {
        node: Node,
    },
    VariableAlreadyExists {
        name: Box<str>,
    },
    VariableDoesNotExist {
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
                GeneratorErrors::InvalidNode { node } => {
                    format!("Invalid node: {:?}", node).as_str().red()
                }
                GeneratorErrors::VariableAlreadyExists { name } => {
                    format!("Variable `{}` already exists", name).as_str().red()
                }
                GeneratorErrors::VariableDoesNotExist { name } => {
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

impl std::error::Error for GeneratorErrors {}
