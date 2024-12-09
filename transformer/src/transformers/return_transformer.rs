use errors::TransformerErrors;
use nilang_types::nodes::Node;

use crate::{temporaries::Temporaries, Instruction};

pub fn transform_return(
    temporaries: &mut Temporaries,

    node: Node,
) -> Result<Vec<Instruction>, TransformerErrors> {
    let function_return_type = temporaries.type_of("@current_function")?;

    match node {
        Node::Number(number) => Ok(vec![Instruction::ReturnNumber(number)]),
        Node::VariableReference(variable_name) => {
            let variable_type = temporaries.type_of(&variable_name)?;

            if variable_type != function_return_type {
                Err(TransformerErrors::InvalidType {
                    expected: function_return_type.into(),
                    received: variable_type.into(),
                })?
            }

            Ok(vec![Instruction::ReturnVariable(variable_name)])
        }
        Node::FieldAccess { structure, field } => match *structure {
            Node::VariableReference(variable_name) => {
                let temp = <Box<str>>::from(format!("{}.{}", variable_name, field));
                let field_type = temporaries.type_of(&temp)?;

                if field_type != function_return_type {
                    Err(TransformerErrors::InvalidType {
                        expected: function_return_type.into(),
                        received: field_type.into(),
                    })?
                }

                Ok(vec![Instruction::ReturnVariable(temp)])
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
