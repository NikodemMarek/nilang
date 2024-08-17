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
