mod field_access_transformator;
mod function_call_transformer;
mod object_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;
pub mod variable_reference_transformer;

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
