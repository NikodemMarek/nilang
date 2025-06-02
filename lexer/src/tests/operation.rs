use nilang_types::{nodes::Operator, tokens::TokenType};

use crate::lex;

#[test]
fn operation() {
    let mut iter = lex("5+4");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Literal("5".into()),
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Operator(Operator::Add),
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Literal("4".into()),
    );

    let mut iter = lex("5.5 * 8");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Literal("5.5".into()),
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Operator(Operator::Multiply),
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Literal("8".into()),
    );
}
