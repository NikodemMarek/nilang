use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_operators() {
    assert_eq!(
        &lex("  +").unwrap(),
        &[Token {
            token: TokenType::Operator,
            value: "+".to_string(),
            start: (0, 2),
            end: (0, 2),
        }]
    );

    assert_eq!(
        &lex(" - ").unwrap(),
        &[Token {
            token: TokenType::Operator,
            value: "-".to_string(),
            start: (0, 1),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("*").unwrap(),
        &[Token {
            token: TokenType::Operator,
            value: "*".to_string(),
            start: (0, 0),
            end: (0, 0),
        }]
    );

    assert_eq!(
        &lex("/").unwrap(),
        &[Token {
            token: TokenType::Operator,
            value: "/".to_string(),
            start: (0, 0),
            end: (0, 0),
        }]
    );

    assert_eq!(
        &lex("%").unwrap(),
        &[Token {
            token: TokenType::Operator,
            value: "%".to_string(),
            start: (0, 0),
            end: (0, 0),
        }]
    );
}
