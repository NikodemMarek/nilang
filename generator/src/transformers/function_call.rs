use nilang_parser::nodes::Node;

use super::{scope::Scope, transform};

pub fn transform_function_call(a: &Node, scope: &mut Scope) -> eyre::Result<Vec<String>> {
    if let Node::FunctionCall { name, arguments } = a {
        let mut code = Vec::new();

        for argument in arguments {
            code.append(&mut transform(argument, scope)?);
        }

        code.push(format!("call _{}", name));

        Ok(code)
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}
