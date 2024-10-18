use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn operation() {
    let mut iter = lex("5+4");
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal,
            value: "5".to_string(),
            start: (0, 0),
            end: (0, 0),
        },
    );
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "+".to_string(),
            start: (0, 1),
            end: (0, 1),
        },
    );
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal,
            value: "4".to_string(),
            start: (0, 2),
            end: (0, 2),
        },
    );

    let mut iter = lex("5.5 * 8");
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal,
            value: "5.5".to_string(),
            start: (0, 0,),
            end: (0, 2),
        },
    );
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "*".to_string(),
            start: (0, 4),
            end: (0, 4),
        },
    );
    assert_eq!(
        iter.next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal,
            value: "8".to_string(),
            start: (0, 6),
            end: (0, 6),
        },
    );
}
