use nilang_parser::nodes::{Node, Operator};

use crate::transformers::transform;

pub fn transform_operation(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Operation { operator, a, b } = a {
        let (a_data, a_code) = transform(a);
        let (b_data, b_code) = transform(b);
        let (a_code, b_code) = (a_code.as_slice(), b_code.as_slice());

        (
            [a_data, b_data].concat(),
            match operator {
                Operator::Add => [
                    a_code,
                    b_code,
                    &[
                        String::from("pop %rbx"),
                        String::from("pop %rax"),
                        String::from("add %rbx, %rax"),
                        String::from("push %rax"),
                    ],
                ]
                .concat(),
                Operator::Subtract => [
                    a_code,
                    b_code,
                    &[
                        String::from("pop %rbx"),
                        String::from("pop %rax"),
                        String::from("sub %rbx, %rax"),
                        String::from("push %rax"),
                    ],
                ]
                .concat(),
                Operator::Multiply => [
                    a_code,
                    b_code,
                    &[
                        String::from("pop %rbx"),
                        String::from("pop %rax"),
                        String::from("imul %rbx, %rax"),
                        String::from("push %rax"),
                    ],
                ]
                .concat(),
                Operator::Divide => [
                    a_code,
                    b_code,
                    &[
                        String::from("pop %rbx"),
                        String::from("pop %rax"),
                        String::from("cltd"),
                        String::from("idiv %rbx"),
                        String::from("push %rax"),
                    ],
                ]
                .concat(),
                Operator::Modulo => [
                    a_code,
                    b_code,
                    &[
                        String::from("pop %rbx"),
                        String::from("pop %rax"),
                        String::from("cltd"),
                        String::from("idiv %rbx"),
                        String::from("push %rax"),
                    ],
                ]
                .concat(),
            },
        )
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformers::operator::transform_operation;
    use nilang_parser::nodes::{Node, Operator};

    #[test]
    fn add_numbers() {
        let node = Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("add %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn subtract_numbers() {
        let node = Node::Operation {
            operator: Operator::Subtract,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("sub %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn multiply_numbers() {
        let node = Node::Operation {
            operator: Operator::Multiply,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("imul %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn divide_numbers() {
        let node = Node::Operation {
            operator: Operator::Divide,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn modulo_numbers() {
        let node = Node::Operation {
            operator: Operator::Modulo,
            a: Box::new(Node::Number(1.)),
            b: Box::new(Node::Number(2.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn add_operation() {
        let node = Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
            b: Box::new(Node::Number(3.)),
        };
        let (data, code) = transform_operation(&node);

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
                String::from("push $3"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("add %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn subtract_operation() {
        let node = Node::Operation {
            operator: Operator::Subtract,
            a: Box::new(Node::Operation {
                operator: Operator::Subtract,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
            b: Box::new(Node::Number(3.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("sub %rbx, %rax"),
                String::from("push %rax"),
                String::from("push $3"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("sub %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn multiply_operation() {
        let node = Node::Operation {
            operator: Operator::Multiply,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
            b: Box::new(Node::Number(3.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("imul %rbx, %rax"),
                String::from("push %rax"),
                String::from("push $3"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("imul %rbx, %rax"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn divide_operation() {
        let node = Node::Operation {
            operator: Operator::Divide,
            a: Box::new(Node::Operation {
                operator: Operator::Divide,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
            b: Box::new(Node::Number(3.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax"),
                String::from("push $3"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax")
            ])
        );
    }

    #[test]
    fn modulo_operation() {
        let node = Node::Operation {
            operator: Operator::Modulo,
            a: Box::new(Node::Operation {
                operator: Operator::Modulo,
                a: Box::new(Node::Number(1.)),
                b: Box::new(Node::Number(2.)),
            }),
            b: Box::new(Node::Number(3.)),
        };
        let (data, code) = transform_operation(&node);

        assert_eq!(data, Vec::<String>::new());
        assert_eq!(
            code,
            Vec::from([
                String::from("push $1"),
                String::from("push $2"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax"),
                String::from("push $3"),
                String::from("pop %rbx"),
                String::from("pop %rax"),
                String::from("cltd"),
                String::from("idiv %rbx"),
                String::from("push %rax")
            ])
        );
    }
}
