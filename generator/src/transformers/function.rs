use nilang_parser::nodes::Node;

use crate::{transformers::transform, utils::generate_function};

use super::scope::Scope;

pub fn transform_function(a: &Node, scope: &mut Scope) -> (Vec<String>, Vec<String>) {
    if let Node::FunctionDeclaration {
        name,
        parameters: _,
        body,
    } = a
    {
        let this = transform(body, scope);
        (this.0, generate_function(name, &this.1))
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::{function::transform_function, scope::Scope};
    use nilang_parser::nodes::Node;

    #[test]
    fn function() {
        let node = Node::FunctionDeclaration {
            name: String::from("main"),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from([Node::Return(Box::new(
                Node::Number(6.),
            ))]))),
        };
        let (data, code) = transform_function(&node, &mut Scope::default());

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    pushq %rbp"),
                String::from("    movq %rsp, %rbp"),
                String::from("    movq $6, %rbx"),
                String::from("    leave"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }
}
