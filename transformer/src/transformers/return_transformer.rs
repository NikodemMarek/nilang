use errors::TransformerErrors;
use nilang_types::nodes::Node;

use crate::{temporaries::Temporaries, Instruction};

use super::function_call_transformer::transform_function_call;

pub fn transform_return(
    context: &std::collections::HashMap<
        Box<str>,
        (
            Box<str>,
            std::collections::HashMap<Box<str>, Box<str>>,
            Vec<Node>,
        ),
    >,
    temporaries: &mut Temporaries,

    node: Node,
    return_type: Box<str>,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        Node::Number(number) => Ok(vec![Instruction::ReturnNumber(number)]),
        Node::VariableReference(variable_name) => {
            let variable_type = temporaries.access(&variable_name)?;

            if *variable_type != *return_type {
                Err(TransformerErrors::InvalidType {
                    expected: return_type.into(),
                    received: variable_type.into(),
                })?
            }

            Ok(vec![Instruction::ReturnVariable(variable_name)])
        }
        Node::FieldAccess { structure, field } => match *structure {
            Node::VariableReference(variable_name) => {
                let temp = <Box<str>>::from(format!("{}.{}", variable_name, field));
                let field_type = temporaries.access(&temp)?;

                if *field_type != *return_type {
                    Err(TransformerErrors::InvalidType {
                        expected: return_type.into(),
                        received: field_type.into(),
                    })?
                }

                Ok(vec![Instruction::ReturnVariable(temp)])
            }
            _ => unimplemented!(),
        },
        Node::FunctionCall { name, arguments } => {
            let (instructions, return_temporary) =
                transform_function_call(context, temporaries, name, &arguments, return_type)?;
            Ok([
                instructions,
                vec![Instruction::ReturnVariable(return_temporary)],
            ]
            .concat())
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nilang_types::nodes::Node;

    #[test]
    fn test_transform_return_number() {
        let mut temporaries = Temporaries::default();
        let node = Node::Number(42.0);
        let function_return_type = "int";
        let result = transform_return(&mut temporaries, node, function_return_type.into()).unwrap();
        assert_eq!(result, [Instruction::ReturnNumber(42.0)]);
    }

    #[test]
    fn test_transform_return_variable() {
        let mut temporaries = Temporaries::default();
        temporaries.declare("x".into(), "int".into());
        let node = Node::VariableReference("x".into());
        let function_return_type = "int";
        let result = transform_return(&mut temporaries, node, function_return_type.into()).unwrap();
        assert_eq!(result, [Instruction::ReturnVariable("x".into())]);
    }

    #[test]
    fn test_transform_return_field_access() {
        let mut temporaries = Temporaries::default();
        temporaries.declare("x".into(), "struct".into());
        temporaries.declare("x.y".into(), "int".into());
        let node = Node::FieldAccess {
            structure: Box::new(Node::VariableReference("x".into())),
            field: "y".into(),
        };
        let function_return_type = "int";
        let result = transform_return(&mut temporaries, node, function_return_type.into()).unwrap();
        assert_eq!(result, [Instruction::ReturnVariable("x.y".into())]);
    }
}
