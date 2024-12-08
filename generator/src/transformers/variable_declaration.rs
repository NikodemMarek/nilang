use errors::GeneratorErrors;
use nilang_types::nodes::Node;

use crate::{utils::generate_allocation, TypesRef};

use super::{
    function_call::transform_function_call, operator::transform_operation, scope::Scope, transform,
};

pub fn transform_variable_declaration(
    node: &Node,
    tr: &TypesRef,
    scope: &mut Scope,
) -> eyre::Result<Vec<String>> {
    if let Node::VariableDeclaration {
        name,
        value,
        r#type,
    } = node
    {
        match *value.to_owned() {
            Node::Number(num) => Ok(Vec::from([format!(
                "movq ${}, -{}(%rbp)",
                num,
                scope.insert(name, r#type)?
            )])),
            Node::VariableReference(reference_name) => Ok(Vec::from([
                format!("movq -{}(%rbp), %rax", scope.get(&reference_name)?),
                format!("movq %rax, -{}(%rbp)", scope.insert(name, r#type)?),
            ])),
            node @ Node::Operation { .. } => Ok([
                transform_operation(&node, scope, "%rax")?,
                Vec::from([format!("movq %rax, -{}(%rbp)", scope.insert(name, r#type)?)]),
            ]
            .concat()),
            node @ Node::FunctionCall { .. } => Ok([
                transform_function_call(&node, scope)?,
                Vec::from([format!("movq %rbx, -{}(%rbp)", scope.insert(name, r#type)?)]),
            ]
            .concat()),
            Node::Object { structure, fields } => {
                if structure != *r#type {
                    Err(GeneratorErrors::InvalidType {
                        expected: r#type.clone(),
                        received: structure.clone(),
                    })?;
                }

                let size = tr.get_structure_size(&structure)?;

                Ok([
                    generate_allocation(size),
                    Vec::from([format!("movq %rax, -{}(%rbp)", scope.insert(name, r#type)?)]),
                    fields
                        .iter()
                        .map(|(name, value)| {
                            let value = match value {
                                Node::Number(value) => format!("${}", value),
                                _ => todo!(),
                            };
                            tr.get_field_offset(r#type, name)
                                .map(|offset| format!("movq {}, -{}(%rax)", value, offset))
                        })
                        .collect::<Result<Vec<String>, GeneratorErrors>>()?,
                ]
                .concat())
            }
            Node::FieldAccess { .. } => todo!(),
            node @ Node::Scope(_)
            | node @ Node::FunctionDeclaration { .. }
            | node @ Node::VariableDeclaration { .. }
            | node @ Node::Return(_)
            | node @ Node::Structure { .. } => Err(GeneratorErrors::InvalidNode { node })?,
        }
    } else {
        panic!("Unexpected node: {:?}", node)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::TypesRef;

    use super::transform_variable_declaration;

    #[test]
    fn variable_declaration_with_number() {
        assert_eq!(
            transform_variable_declaration(
                &(Node::VariableDeclaration {
                    name: "a".into(),
                    r#type: "int".into(),
                    value: Box::new(Node::Number(42.)),
                }),
                &TypesRef::default(),
                &mut super::Scope::default(),
            )
            .unwrap(),
            [String::from("movq $42, -8(%rbp)")]
        );
    }

    #[test]
    fn variable_declaration_with_reference() {
        let mut scope = super::Scope::default();
        let _ = scope.insert("b", "int");

        assert_eq!(
            transform_variable_declaration(
                &(Node::VariableDeclaration {
                    name: "a".into(),
                    r#type: "int".into(),
                    value: Box::new(Node::VariableReference("b".into())),
                }),
                &TypesRef::default(),
                &mut scope,
            )
            .unwrap(),
            [
                String::from("movq -8(%rbp), %rax"),
                String::from("movq %rax, -16(%rbp)")
            ]
        );
    }

    #[test]
    fn variable_declaration_with_operation() {
        assert_eq!(
            transform_variable_declaration(
                &(Node::VariableDeclaration {
                    name: "a".into(),
                    r#type: "int".into(),
                    value: Box::new(Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(1.)),
                        b: Box::new(Node::Number(2.)),
                    }),
                }),
                &TypesRef::default(),
                &mut super::Scope::default(),
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("add $2, %rax"),
                String::from("movq %rax, -8(%rbp)")
            ]
        );
    }
}
