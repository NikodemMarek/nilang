mod field_access_transformator;
mod function_call_transformer;
mod object_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;
mod variable_reference_transformer;

use std::iter::{empty, once};

use errors::TransformerErrors;

use field_access_transformator::transform_field_access;
use function_call_transformer::transform_function_call;
use nilang_types::nodes::{ExpressionNode, FunctionCall, StatementNode};
use object_transformer::transform_object;
use operation_transformer::transform_operation;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;
use variable_reference_transformer::transform_variable_reference;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, StructuresRef, Type};

pub fn transform_statement<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    node: StatementNode,
    return_type: &Type,
    temporaries: &'a Temporaries,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    match node {
        StatementNode::Return(node) => transform_return(context, temporaries, *node, return_type),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(context, temporaries, name, &r#type, *value),
        StatementNode::FunctionCall(FunctionCall { name, arguments }) => transform_function_call(
            context,
            temporaries,
            name,
            &arguments,
            "".into(),
            &Type::Void,
        ),
    }
}

pub fn transform_expression<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,

    node: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    match node {
        ExpressionNode::Number(number) => {
            Box::new(once(Ok(Instruction::LoadNumber(result, number))))
        }
        ExpressionNode::Char(char) => Box::new(once(Ok(Instruction::LoadChar(result, char)))),
        ExpressionNode::String(_) => todo!(),
        ExpressionNode::Object { r#type, fields } => {
            transform_object(context, temporaries, fields, result, &r#type)
        }
        ExpressionNode::VariableReference(variable) => {
            transform_variable_reference(context, temporaries, variable, result, r#type)
        }
        ExpressionNode::FieldAccess { structure, field } => {
            transform_field_access(context, temporaries, *structure, field, result, r#type)
        }
        ExpressionNode::Operation { operator, a, b } => {
            transform_operation(context, temporaries, operator, *a, *b, result, r#type)
        }
        ExpressionNode::FunctionCall(FunctionCall { name, arguments }) => {
            transform_function_call(context, temporaries, name, &arguments, result, r#type)
        }
    }
}

pub fn copy_all_fields<'a>(
    context: &(FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,

    source: Box<str>,
    destination: Box<str>,

    object_type: &Type,
) -> Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a> {
    let object_type = match object_type {
        Type::Object(object_type) => object_type,
        Type::Void => return Box::new(empty()),
        Type::Int | Type::Char => {
            return Box::new(once(Ok(Instruction::Copy(destination, source))));
        }
    };

    let Ok(object_fields_from_to) =
        object_fields_from_to(&context.1, source, destination, object_type)
    else {
        return Box::new(once(Err(TransformerErrors::TypeNotFound {
            name: object_type.clone(),
        })));
    };

    let instructions = object_fields_from_to.into_iter().flat_map(
        |(destination_temporary, source_temporary, field_type)| {
            temporaries.declare_named(source_temporary.clone(), field_type);

            once(Ok(Instruction::Declare(destination_temporary.clone()))).chain(Ok::<
                Result<Instruction, TransformerErrors>,
                TransformerErrors,
            >(
                temporaries
                    .access(&source_temporary.clone())
                    .map(|_| Instruction::Copy(destination_temporary, source_temporary)),
            ))
        },
    );

    Box::new(instructions)
}

pub fn object_fields_from_to(
    context: &StructuresRef,

    source: Box<str>,
    destination: Box<str>,

    object_type: &str,
) -> Result<Vec<(Box<str>, Box<str>, Type)>, TransformerErrors> {
    Ok(context
        .get_fields_flattened(object_type)?
        .iter()
        .map(|(field, field_type)| {
            let destination_temporary = <Box<str>>::from(format!("{}.{}", destination, field));
            let source_temporary = <Box<str>>::from(format!("{}.{}", source, field));
            (destination_temporary, source_temporary, field_type.clone())
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nilang_types::nodes::StructureDeclaration;

    use crate::{transformers::object_fields_recursive, StructuresRef};

    #[test]
    fn test_object_fields_recursive() {
        let types_ref = StructuresRef::from(
            [
                StructureDeclaration {
                    name: "Point".into(),
                    fields: HashMap::from([("x".into(), "int".into()), ("y".into(), "int".into())]),
                },
                StructureDeclaration {
                    name: "Rect".into(),
                    fields: HashMap::from([
                        ("start".into(), "Point".into()),
                        ("end".into(), "Point".into()),
                    ]),
                },
                StructureDeclaration {
                    name: "Label".into(),
                    fields: HashMap::from([
                        ("text".into(), "char".into()),
                        ("anchor".into(), "Point".into()),
                    ]),
                },
            ]
            .as_ref(),
        );

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
                ("text".to_string(), "char".into()),
            ],
        );
    }
}
