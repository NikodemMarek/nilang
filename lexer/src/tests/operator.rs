use nilang_types::{
    nodes::expressions::{Arithmetic, Operator},
    tokens::{Token, TokenType},
};

use crate::lex;

#[test]
fn arithmetic_operator() {
    assert_eq!(
        lex("  +").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Add)),
            start: (0, 2),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex(" - ").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Subtract)),
            start: (0, 1),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("*").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Multiply)),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("/").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Divide)),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("%").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Modulo)),
            start: (0, 0),
            end: (0, 0),
        }
    );
}

#[test]
fn boolean_operator() {
    assert_eq!(
        lex("==").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::Equal)),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("!=").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::NotEqual)),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("<").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::Less)),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex(">").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::More)),
            start: (0, 0),
            end: (0, 0),
        }
    );
    assert_eq!(
        lex("<=").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::LessOrEqual)),
            start: (0, 0),
            end: (0, 1),
        }
    );
    assert_eq!(
        lex(">=").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Boolean(Boolean::MoreOrEqual)),
            start: (0, 0),
            end: (0, 1),
        }
    );
}
