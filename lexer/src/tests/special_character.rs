use nilang_types::{nodes::Operator, tokens::TokenType};

use crate::lex;

#[test]
fn special_character() {
    let mut iter = lex(" (5)");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::OpeningParenthesis,
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Literal("5".into()),
    );

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::ClosingParenthesis,
    );

    let mut iter = lex("(5 + 4)");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::OpeningParenthesis,
    );

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

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::ClosingParenthesis,
    );

    let mut iter = lex("a = b");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Identifier("a".into()),
    );

    assert_eq!(*iter.next().unwrap().unwrap(), TokenType::Equals,);

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Identifier("b".into()),
    );

    let mut iter = lex("a: b");

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Identifier("a".into()),
    );

    assert_eq!(*iter.next().unwrap().unwrap(), TokenType::Colon,);

    assert_eq!(
        *iter.next().unwrap().unwrap(),
        TokenType::Identifier("b".into()),
    );
}
