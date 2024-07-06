use crate::{
    extend_operation,
    nodes::{Node, Operator},
};

#[test]
fn extend_complex_operation() {
    assert_eq!(
        extend_operation(
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            },
            Operator::Add,
            Node::Number(4.)
        ),
        Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            }),
            b: Box::new(Node::Number(4.))
        }
    );
    assert_eq!(
        extend_operation(
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            },
            Operator::Multiply,
            Node::Number(4.)
        ),
        Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Number(6.)),
            b: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(8.)),
                b: Box::new(Node::Number(4.))
            })
        }
    );
    assert_eq!(
        extend_operation(
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            },
            Operator::Add,
            Node::Number(4.)
        ),
        Node::Operation {
            operator: Operator::Add,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            }),
            b: Box::new(Node::Number(4.))
        }
    );
    assert_eq!(
        extend_operation(
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            },
            Operator::Multiply,
            Node::Number(4.)
        ),
        Node::Operation {
            operator: Operator::Multiply,
            a: Box::new(Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            }),
            b: Box::new(Node::Number(4.))
        }
    );
    assert_eq!(
        extend_operation(
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            },
            Operator::Subtract,
            Node::Scope(vec![Node::Return(Box::new(Node::Number(4.)))]),
        ),
        Node::Operation {
            operator: Operator::Subtract,
            a: Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(8.))
            }),
            b: Box::new(Node::Scope(vec![Node::Return(Box::new(Node::Number(4.)))]))
        }
    );
}
