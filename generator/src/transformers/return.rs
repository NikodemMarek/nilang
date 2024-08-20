use nilang_parser::nodes::Node;

use crate::transformers::transform;

pub fn transform_return(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Return(inner) = a {
        let transformed = transform(inner);
        (transformed.0, {
            [
                transformed.1.as_slice(),
                &[String::from("pop %rax"), String::from("movl %eax, %ebx")],
            ]
            .concat()
        })
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::r#return::transform_return;
    use nilang_parser::nodes::Node;

    #[test]
    fn return_number() {
        let node = Node::Return(Box::new(Node::Number(42.)));
        let (data, code) = transform_return(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $42"),
                String::from("pop %rax"),
                String::from("movl %eax, %ebx")
            ])
        );
    }

    #[test]
    fn return_operation() {
        let node = Node::Return(Box::new(Node::Operation {
            operator: nilang_parser::nodes::Operator::Add,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        }));
        let (data, code) = transform_return(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("add %rbx, %rax"),
                String::from("push %rax"),
                String::from("pop %rax"),
                String::from("movl %eax, %ebx")
            ])
        );
    }
}
