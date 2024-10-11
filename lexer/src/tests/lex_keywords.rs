use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_keywords() {
    assert_eq!(
        &lex("fn").unwrap(),
        &[Token {
            token: TokenType::Keyword,
            value: "fn".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("rt").unwrap(),
        &[Token {
            token: TokenType::Keyword,
            value: "rt".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("vr").unwrap(),
        &[Token {
            token: TokenType::Keyword,
            value: "vr".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );
}
