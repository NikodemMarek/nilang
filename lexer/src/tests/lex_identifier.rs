use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn lex_identifier() {
    assert_eq!(
        &lex("fn").unwrap(),
        &[Token {
            token: TokenType::Identifier,
            value: "fn".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("rt").unwrap(),
        &[Token {
            token: TokenType::Identifier,
            value: "rt".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("vr").unwrap(),
        &[Token {
            token: TokenType::Identifier,
            value: "vr".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );
}
