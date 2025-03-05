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
    context: &std::collections::HashMap<
        Box<str>,
        (
            Box<str>,
            std::collections::HashMap<Box<str>, Box<str>>,
            Vec<Node>,
        ),
    >,
    node: Node,
    return_type: Box<str>,
    temporaries: &mut Temporaries,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        Node::Return(node) => transform_return(context, temporaries, *node, return_type),
        Node::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(temporaries, name, r#type, *value),
        Node::FunctionCall { name, arguments } => {
            transform_function_call(context, temporaries, name, &arguments, "void".into())
                .map(|(instructions, _)| instructions)
        }
        _ => unimplemented!(),
    }
}
