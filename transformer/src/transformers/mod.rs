mod function_call_transformer;
mod return_transformer;
mod variable_declaration_transformer;

use errors::TransformerErrors;

use nilang_types::nodes::StatementNode;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction};

pub fn transform_statement(
    context: &FunctionsRef,
    node: StatementNode,
    return_type: Box<str>,
    temporaries: &mut Temporaries,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        StatementNode::Return(node) => transform_return(context, temporaries, *node, return_type),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(temporaries, name, r#type, *value),
    }
}
