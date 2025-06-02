use nilang_types::{nodes::Operator, tokens::TokenType};

use crate::lex;

#[test]
fn operator() {
    assert_eq!(
        *lex("  +").next().unwrap().unwrap(),
        TokenType::Operator(Operator::Add),
    );

    assert_eq!(
        *lex(" - ").next().unwrap().unwrap(),
        TokenType::Operator(Operator::Subtract),
    );

    assert_eq!(
        *lex("*").next().unwrap().unwrap(),
        TokenType::Operator(Operator::Multiply),
    );

    assert_eq!(
        *lex("/").next().unwrap().unwrap(),
        TokenType::Operator(Operator::Divide),
    );

    assert_eq!(
        *lex("%").next().unwrap().unwrap(),
        TokenType::Operator(Operator::Modulo),
    );
}
