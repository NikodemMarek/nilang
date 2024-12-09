mod function_call_transformer;
mod return_transformer;
mod variable_declaration_transformer;

use errors::TransformerErrors;
use function_call_transformer::transform_function_call;
use nilang_types::nodes::Node;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;

use crate::{temporaries::Temporaries, Instruction};

pub fn transform(
    node: Node,
    temporaries: &mut Temporaries,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        Node::Return(node) => transform_return(temporaries, *node),
        Node::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(temporaries, name, r#type, *value),
        Node::FunctionCall { name, arguments } => {
            transform_function_call(temporaries, name, &arguments)
        }
        _ => unimplemented!(),
    }
}
