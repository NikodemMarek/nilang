mod field_access_transformator;
mod function_call_transformer;
mod object_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;
mod variable_reference_transformer;

use std::iter::once;

use field_access_transformator::transform_field_access;
use function_call_transformer::transform_function_call;
use nilang_types::nodes::{ExpressionNode, FunctionCall, StatementNode};
use object_transformer::transform_object;
use operation_transformer::transform_operation;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;
use variable_reference_transformer::transform_variable_reference;

use crate::{Context, Instruction, InstructionsIterator, Type};

pub fn transform_statement<'a>(
    context: &'a Context,

    node: StatementNode,
    return_type: &Type,
) -> InstructionsIterator<'a> {
    match node {
        StatementNode::Return(node) => transform_return(context, *node, return_type),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(context, name, &r#type, *value),
        StatementNode::FunctionCall(FunctionCall { name, arguments }) => {
            transform_function_call(context, name, &arguments, "".into(), &Type::Void)
        }
    }
}

pub fn transform_expression<'a>(
    context: &'a Context,

    node: ExpressionNode,

    result: Box<str>,
    r#type: &Type,
) -> InstructionsIterator<'a> {
    match node {
        ExpressionNode::Boolean(boolean) => {
            Box::new(once(Ok(Instruction::LoadBoolean(result, boolean))))
        }
        ExpressionNode::Number(number) => {
            Box::new(once(Ok(Instruction::LoadNumber(result, number))))
        }
        ExpressionNode::Char(char) => Box::new(once(Ok(Instruction::LoadChar(result, char)))),
        ExpressionNode::String(text) => transform_string_declaration(context, &text, result),
        ExpressionNode::Object { r#type, fields } => {
            transform_object(context, fields, result, &r#type)
        }
        ExpressionNode::VariableReference(variable) => {
            transform_variable_reference(context, variable, result, r#type)
        }
        ExpressionNode::FieldAccess { structure, field } => {
            transform_field_access(context, *structure, field, result, r#type)
        }
        ExpressionNode::Operation { operator, a, b } => {
            transform_operation(context, operator, *a, *b, result, r#type)
        }
        ExpressionNode::FunctionCall(FunctionCall { name, arguments }) => {
            transform_function_call(context, name, &arguments, result, r#type)
        }
    }
}

fn transform_string_declaration<'a>(
    Context {
        temporaries, data, ..
    }: &'a Context,

    text: &str,
    result: Box<str>,
) -> InstructionsIterator<'a> {
    let size_temporary = temporaries.declare(Type::Int);
    let name = <Box<str>>::from(format!("string__{}", result));

    data.borrow_mut().push((name.clone(), text.into()));

    Box::new(
        [
            Ok(Instruction::Declare(size_temporary.clone())),
            Ok(Instruction::LoadNumber(
                size_temporary.clone(),
                text.len() as f64,
            )),
            Ok(Instruction::LoadStringLocation(result.clone(), name)),
        ]
        .into_iter(),
    )
}
