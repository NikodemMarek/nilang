use nilang_parser::nodes::Node;

use crate::{transformers::transform, utils::generate_function};

pub fn transform_function(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Function {
        name,
        parameters: _,
        body,
    } = a
    {
        let this = transform(body);
        (this.0, generate_function(name, &this.1))
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::function::transform_function;
    use nilang_parser::nodes::Node;

    #[test]
    fn function() {
        let node = Node::Function {
            name: String::from("main"),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from([Node::Return(Box::new(
                Node::Number(6.),
            ))]))),
        };
        let (data, code) = transform_function(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    push $6"),
                String::from("    pop %rax"),
                String::from("    movl %eax, %ebx"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }
}
