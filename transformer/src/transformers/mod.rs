mod field_access_transformator;
mod function_call_transformer;
mod object_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;
mod variable_reference_transformer;

use std::iter::once;

use errors::TransformerErrors;

use field_access_transformator::transform_field_access;
use function_call_transformer::transform_function_call;
use nilang_types::nodes::{ExpressionNode, FunctionCall, StatementNode};
use object_transformer::transform_object;
use operation_transformer::transform_operation;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;
use variable_reference_transformer::transform_variable_reference;

use crate::{
    structures_ref::StructuresRef, temporaries::Temporaries, FunctionsRef, Instruction, Type,
};

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
