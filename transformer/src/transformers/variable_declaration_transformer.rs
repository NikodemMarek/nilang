use errors::TransformerErrors;
use nilang_types::nodes::ExpressionNode;

use crate::{temporaries::Temporaries, Instruction};

pub fn transform_variable_declaration(
    temporaries: &mut Temporaries,

    name: Box<str>,
    r#type: Box<str>,
    value: ExpressionNode,
) -> Result<Vec<Instruction>, TransformerErrors> {
    temporaries.declare(name.clone(), r#type.clone());

    match value {
        ExpressionNode::Number(number) => {
            if r#type != "int".into() {
                return Err(TransformerErrors::InvalidType {
                    expected: "int".into(),
                    received: r#type,
                });
            }

            Ok(vec![Instruction::LoadNumber(number, name)])
        }
        ExpressionNode::VariableReference(reference_name) => {
            let reference_type = temporaries.access(&reference_name)?;

            if *reference_type != *r#type {
                Err(TransformerErrors::InvalidType {
                    expected: r#type,
                    received: reference_type.into(),
                })?
            }

            Ok(vec![Instruction::Copy(name, reference_name)])
        }
        ExpressionNode::Object {
            r#type: object_type,
            fields,
        } => {
            if r#type != object_type {
                Err(TransformerErrors::InvalidType {
                    expected: r#type.clone(),
                    received: object_type,
                })?
            }

            let assignments = fields
                .iter()
                .map(|(field_name, field_value)| match *field_value {
                    ExpressionNode::Number(number) => {
                        let temp = <Box<str>>::from(format!("{}.{}", name, field_name));
                        temporaries.declare(temp.clone(), "int".into());
                        Instruction::LoadNumber(number, temp)
                    }
                    _ => unimplemented!(),
                })
                .collect();

            Ok([vec![Instruction::Allocate(name, r#type)], assignments].concat())
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut temporaries = Temporaries::default();
        let result = transform_variable_declaration(
            &mut temporaries,
            "a".into(),
            "int".into(),
            ExpressionNode::Number(10.),
        )
        .unwrap();
        assert_eq!(result, [Instruction::LoadNumber(10., "a".into())]);

        let result = transform_variable_declaration(
            &mut temporaries,
            "b".into(),
            "int".into(),
            ExpressionNode::VariableReference("a".into()),
        )
        .unwrap();
        assert_eq!(result, &[Instruction::Copy("b".into(), "a".into())]);

        let result = transform_variable_declaration(
            &mut temporaries,
            "c".into(),
            "int".into(),
            ExpressionNode::Object {
                r#type: "int".into(),
                fields: vec![("x".into(), ExpressionNode::Number(10.))]
                    .into_iter()
                    .collect(),
            },
        )
        .unwrap();
        assert_eq!(
            result,
            &[
                Instruction::Allocate("c".into(), "int".into()),
                Instruction::LoadNumber(10., "c.x".into())
            ]
        );
    }
}
