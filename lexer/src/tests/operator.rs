use nilang_types::{
    nodes::expressions::Operator,
    tokens::{Token, TokenType},
};

use crate::lex;

#[test]
fn operator() {
    assert_eq!(
        lex("  +").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Add),
            start: (0, 2),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex(" - ").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Subtract),
            start: (0, 1),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("*").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Multiply),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("/").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Divide),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("%").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Modulo),
            start: (0, 0),
            end: (0, 0),
        }
    );
}
