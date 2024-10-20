use nilang_types::nodes::{Node, Operator};

use super::scope::Scope;

pub fn transform_operation(
    node: &Node,
    scope: &mut Scope,
    result_register: &str,
) -> eyre::Result<Vec<String>> {
    if let Node::Operation { operator, a, b } = node {
        let (code, second_operator) = match (*a.clone(), *b.clone()) {
            (Node::Number(value_a), Node::Number(value_b)) => (
                Vec::from([format!("movq ${}, {}", value_a, result_register)]),
                format!("${}", value_b),
            ),
            (Node::Number(value_a), Node::VariableReference(name_b)) => (
                Vec::from([format!("movq ${}, {}", value_a, result_register)]),
                format!("{}(%rbp)", scope.get(&name_b)?),
            ),
            (Node::VariableReference(name_a), Node::Number(value_b)) => (
                Vec::from([format!(
                    "movq {}(%rbp), {}",
                    scope.get(&name_a)?,
                    result_register
                )]),
                format!("${}", value_b),
            ),
            (Node::VariableReference(name_a), Node::VariableReference(name_b)) => (
                Vec::from([format!(
                    "movq {}(%rbp), {}",
                    scope.get(&name_a)?,
                    result_register
                )]),
                format!("{}(%rbp)", scope.get(&name_b)?),
            ),
            (operation_a @ Node::Operation { .. }, Node::Number(value_b)) => (
                transform_operation(&operation_a, scope, result_register)?,
                format!("${}", value_b),
            ),
            (operation_a @ Node::Operation { .. }, Node::VariableReference(name_b)) => (
                transform_operation(&operation_a, scope, result_register)?,
                format!("{}(%rbp)", scope.get(&name_b)?),
            ),
            (Node::Number(value_a), operation_b @ Node::Operation { .. }) => {
                let result_pointer_offset_b = scope.insert_unnamed()?;
                (
                    [
                        transform_operation(&operation_b, scope, result_register)?,
                        Vec::from([
                            format!(
                                "movq {}, {}(%rbp)",
                                result_register, result_pointer_offset_b
                            ),
                            format!("movq ${}, {}", value_a, result_register),
                        ]),
                    ]
                    .concat(),
                    format!("{}(%rbp)", result_pointer_offset_b),
                )
            }
            (Node::VariableReference(name_a), operation_b @ Node::Operation { .. }) => {
                let result_pointer_offset_b = scope.insert_unnamed()?;
                (
                    [
                        transform_operation(&operation_b, scope, result_register)?,
                        Vec::from([
                            format!(
                                "movq {}, {}(%rbp)",
                                result_register, result_pointer_offset_b
                            ),
                            format!("movq {}(%rbp), {}", scope.get(&name_a)?, result_register),
                        ]),
                    ]
                    .concat(),
                    format!("{}(%rbp)", result_pointer_offset_b),
                )
            }
            (a @ Node::Operation { .. }, b @ Node::Operation { .. }) => {
                let result_pointer_offset_a = scope.insert_unnamed()?;
                (
                    [
                        transform_operation(&a, scope, result_register)?,
                        Vec::from([format!(
                            "movq {}, {}(%rbp)",
                            result_register, result_pointer_offset_a
                        )]),
                        transform_operation(&b, scope, result_register)?,
                        Vec::from([format!(
                            "movq {}(%rbp), {}",
                            result_pointer_offset_a, result_register
                        )]),
                    ]
                    .concat(),
                    String::from("%rbx"),
                )
            }
            _ => panic!("Invalid node type"),
        };

        Ok([
            code,
            match operator {
                Operator::Add => {
                    Vec::from([format!("add {}, {}", second_operator, result_register)])
                }
                Operator::Subtract => {
                    Vec::from([format!("sub {}, {}", second_operator, result_register)])
                }
                Operator::Multiply => {
                    Vec::from([format!("imul {}, {}", second_operator, result_register)])
                }
                Operator::Divide => Vec::from([
                    format!("movq {}, %rbx", second_operator),
                    String::from("cqto"),
                    String::from("idiv %rbx"),
                    format!("movq %rax, {}", result_register),
                ]),
                Operator::Modulo => Vec::from([
                    format!("movq {}, %rbx", second_operator),
                    String::from("cqto"),
                    String::from("idiv %rbx"),
                    format!("movq %rdx, {}", result_register),
                ]),
            },
        ]
        .concat())
    } else {
        panic!("Invalid node type");
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::nodes::{Node, Operator};

    use crate::transformers::{operator::transform_operation, scope::Scope};

    #[test]
    fn add_numbers() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [String::from("movq $1, %rax"), String::from("add $2, %rax"),]
        );
    }

    #[test]
    fn subtract_numbers() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Subtract,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [String::from("movq $1, %rax"), String::from("sub $2, %rax"),]
        );
    }

    #[test]
    fn multiply_numbers() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [String::from("movq $1, %rax"), String::from("imul $2, %rax"),]
        );
    }

    #[test]
    fn divide_numbers() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Divide,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("movq $2, %rbx"),
                String::from("cqto"),
                String::from("idiv %rbx"),
                String::from("movq %rax, %rax")
            ]
        );
    }

    #[test]
    fn modulo_numbers() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Modulo,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("movq $2, %rbx"),
                String::from("cqto"),
                String::from("idiv %rbx"),
                String::from("movq %rdx, %rax"),
            ]
        );
    }

    #[test]
    fn number_variable_reference() {
        let mut scope = Scope::default();
        let _ = scope.insert("a");

        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::VariableReference(String::from("a"))),
                }),
                &mut scope,
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("add -8(%rbp), %rax"),
            ]
        );
    }

    #[test]
    fn variable_reference_number() {
        let mut scope = Scope::default();
        let _ = scope.insert("a");

        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::VariableReference(String::from("a"))),
                    b: Box::new(Node::Number(2.)),
                }),
                &mut scope,
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq -8(%rbp), %rax"),
                String::from("add $2, %rax"),
            ]
        );
    }

    #[test]
    fn variable_reference_variable_reference() {
        let mut scope = Scope::default();
        let _ = scope.insert("a");
        let _ = scope.insert("b");

        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::VariableReference(String::from("a"))),
                    b: Box::new(Node::VariableReference(String::from("b"))),
                }),
                &mut scope,
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq -8(%rbp), %rax"),
                String::from("add -16(%rbp), %rax"),
            ]
        );
    }

    #[test]
    fn number_operation() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Node::Number(1.)),
                        b: Box::new(Node::Number(2.)),
                    }),
                    b: Box::new(Node::Number(3.)),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("imul $2, %rax"),
                String::from("add $3, %rax"),
            ]
        );
    }

    #[test]
    fn variable_reference_operation() {
        let mut scope = Scope::default();
        let _ = scope.insert("a");

        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::VariableReference(String::from("a"))),
                    b: Box::new(Node::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Node::Number(2.)),
                        b: Box::new(Node::Number(3.)),
                    }),
                }),
                &mut scope,
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $2, %rax"),
                String::from("imul $3, %rax"),
                String::from("movq %rax, -16(%rbp)"),
                String::from("movq -8(%rbp), %rax"),
                String::from("add -16(%rbp), %rax"),
            ]
        );
    }

    #[test]
    fn operation_number() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(1.)),
                    b: Box::new(Node::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Node::Number(2.)),
                        b: Box::new(Node::Number(3.)),
                    }),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $2, %rax"),
                String::from("imul $3, %rax"),
                String::from("movq %rax, -8(%rbp)"),
                String::from("movq $1, %rax"),
                String::from("add -8(%rbp), %rax"),
            ]
        );
    }

    #[test]
    fn operation_variable_reference() {
        let mut scope = Scope::default();
        let _ = scope.insert("a");

        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Node::Number(1.)),
                        b: Box::new(Node::Number(2.)),
                    }),
                    b: Box::new(Node::VariableReference(String::from("a"))),
                }),
                &mut scope,
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("imul $2, %rax"),
                String::from("add -8(%rbp), %rax"),
            ]
        );
    }

    #[test]
    fn operation_operation() {
        assert_eq!(
            transform_operation(
                &(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Node::Number(1.)),
                        b: Box::new(Node::Number(2.)),
                    }),
                    b: Box::new(Node::Operation {
                        operator: Operator::Subtract,
                        a: Box::new(Node::Number(3.)),
                        b: Box::new(Node::Number(4.)),
                    }),
                }),
                &mut Scope::default(),
                "%rax",
            )
            .unwrap(),
            [
                String::from("movq $1, %rax"),
                String::from("imul $2, %rax"),
                String::from("movq %rax, -8(%rbp)"),
                String::from("movq $3, %rax"),
                String::from("sub $4, %rax"),
                String::from("movq -8(%rbp), %rax"),
                String::from("add %rbx, %rax"),
            ]
        );
    }
}
