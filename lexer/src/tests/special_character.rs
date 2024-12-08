use nilang_types::{
    nodes::Operator,
    tokens::{Token, TokenType},
};

use crate::lex;

#[test]
fn special_character() {
    let mut iter = lex(" (5)");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::OpeningParenthesis,
            start: (0, 1),
            end: (0, 1),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("5".into()),
            start: (0, 2),
            end: (0, 2),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::ClosingParenthesis,
            start: (0, 3),
            end: (0, 3),
        },
    );

    let mut iter = lex("(5 + 4)");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::OpeningParenthesis,
            start: (0, 0),
            end: (0, 0),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("5".into()),
            start: (0, 1),
            end: (0, 1),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator(Operator::Add),
            start: (0, 3),
            end: (0, 3),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("4".into()),
            start: (0, 5),
            end: (0, 5),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::ClosingParenthesis,
            start: (0, 6),
            end: (0, 6),
        },
    );

    let mut iter = lex("a = b");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("a".into()),
            start: (0, 0),
            end: (0, 0),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Equals,
            start: (0, 2),
            end: (0, 2),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("b".into()),
            start: (0, 4),
            end: (0, 4),
        },
    );

    let mut iter = lex("a: b");

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("a".into()),
            start: (0, 0),
            end: (0, 0),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Colon,
            start: (0, 1),
            end: (0, 1),
        },
    );

    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("b".into()),
            start: (0, 3),
            end: (0, 3),
        },
    );
}
