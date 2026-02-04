use nilang_types::{
    nodes::expressions::{Arithmetic, Operator},
    tokens::{Token, TokenType},
};

use crate::lex;

#[test]
fn operation() {
    let mut iter = lex("5+4");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("5".into()),
            start: (0, 0),
            end: (0, 0),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Add)),
            start: (0, 1),
            end: (0, 1),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("4".into()),
            start: (0, 2),
            end: (0, 2),
        },
    );

    let mut iter = lex("5.5 * 8");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("5.5".into()),
            start: (0, 0,),
            end: (0, 2),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Multiply)),
            start: (0, 4),
            end: (0, 4),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("8".into()),
            start: (0, 6),
            end: (0, 6),
        },
    );
}
