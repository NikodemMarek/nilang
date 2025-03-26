mod field_access_transformator;
mod function_call_transformer;
mod object_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;
mod variable_reference_transformer;

use errors::TransformerErrors;

use field_access_transformator::transform_field_access;
use function_call_transformer::transform_function_call;
use nilang_types::nodes::{ExpressionNode, StatementNode};
use object_transformer::transform_object;
use operation_transformer::transform_operation;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;
use variable_reference_transformer::transform_variable_reference;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, Type, TypesRef};

pub fn transform_statement(
    context: (&FunctionsRef, &TypesRef),
    node: StatementNode,
    return_type: &Type,
    temporaries: &mut Temporaries,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        StatementNode::Return(node) => transform_return(context, temporaries, *node, return_type),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(context, temporaries, name, &r#type.into(), *value),
    }
}

pub fn transform_expression(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    node: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        ExpressionNode::Number(number) => Ok(vec![Instruction::LoadNumber(result, number)]),
        ExpressionNode::VariableReference(variable) => {
            transform_variable_reference(context, temporaries, variable, result, r#type)
        }
        ExpressionNode::FieldAccess { structure, field } => {
            transform_field_access(context, temporaries, *structure, field, result, r#type)
        }
        ExpressionNode::Operation { operator, a, b } => {
            transform_operation(context, temporaries, operator, *a, *b, result, r#type)
        }
        ExpressionNode::Object { r#type, fields } => {
            transform_object(context, temporaries, fields, result, &r#type.into())
        }
        ExpressionNode::FunctionCall { name, arguments } => {
            transform_function_call(context, temporaries, name, &arguments, result, r#type)
        }
    }
}

pub fn copy_all_fields(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,
    source: Box<str>,
    destination: Box<str>,
    object_type: &Type,
) -> Result<Vec<Instruction>, TransformerErrors> {
    if let Type::Object(object_type) = object_type {
        let mut instructions = Vec::new();
        for (destination_temporary, source_temporary, field_type) in
            object_fields_from_to(context.1, source, destination, object_type)?
        {
            temporaries.declare_named(source_temporary.clone(), field_type);
            instructions.push(Instruction::Copy(destination_temporary, source_temporary));
        }
        return Ok(instructions);
    }

    Ok(vec![Instruction::Copy(destination, source)])
}

pub fn object_fields_from_to(
    context: &TypesRef,

    source: Box<str>,
    destination: Box<str>,

    object_type: &str,
) -> Result<Vec<(Box<str>, Box<str>, Type)>, TransformerErrors> {
    Ok(object_fields_recursive(context, object_type)?
        .iter()
        .map(|(field, field_type)| {
            let destination_temporary = <Box<str>>::from(format!("{}.{}", destination, field));
            let source_temporary = <Box<str>>::from(format!("{}.{}", source, field));
            (destination_temporary, source_temporary, field_type.clone())
        })
        .collect())
}

pub fn object_fields_recursive(
    context: &TypesRef,

    object_type: &str,
) -> Result<Vec<(String, Type)>, TransformerErrors> {
    let fields_map = if let Some(fields) = context.get_fields(object_type) {
        fields
    } else {
        panic!("Type {} not found", object_type);
    };

    let mut fields = Vec::new();
    for (field, field_type) in fields_map {
        if let Type::Object(field_type) = field_type {
            let v = object_fields_recursive(context, field_type)?;

            fields.append(
                &mut v
                    .iter()
                    .map(|(subfield, r#type)| (format!("{}.{}", field, subfield), r#type.clone()))
                    .collect(),
            );
        } else {
            fields.push((field.to_string(), field_type.clone()));
        }
    }

    fields.sort();
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nilang_types::nodes::Structure;

    use crate::{transformers::object_fields_recursive, TypesRef};

    #[test]
    fn test_object_fields_recursive() {
        let types_ref = TypesRef::from(HashMap::from([
            (
                "Point".into(),
                Structure {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), "int".into()), ("y".into(), "int".into())]),
                },
            ),
            (
                "Rect".into(),
                Structure {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), "Point".into()),
                        ("end".into(), "Point".into()),
                    ]),
                },
            ),
            (
                "Label".into(),
                Structure {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), "str".into()),
                        ("anchor".into(), "Point".into()),
                    ]),
                },
            ),
        ]));

        let mut result = object_fields_recursive(&types_ref, "Rect").unwrap();
        result.sort();
        assert_eq!(
            result,
            [
                ("end.x".to_string(), "int".into()),
                ("end.y".to_string(), "int".into()),
                ("start.x".to_string(), "int".into()),
                ("start.y".to_string(), "int".into()),
            ],
        );

        let result = object_fields_recursive(&types_ref, "Label").unwrap();
        assert_eq!(
            result,
            [
                ("anchor.x".to_string(), "int".into()),
                ("anchor.y".to_string(), "int".into()),
                ("text".to_string(), "str".into()),
            ],
        );
    }
}
