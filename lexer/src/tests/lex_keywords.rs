use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_keywords() {
    assert_eq!(
        &lex("fn"),
        &[Token {
            token: TokenType::Keyword,
            value: "fn".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        &lex("rt"),
        &[Token {
            token: TokenType::Keyword,
            value: "rt".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        &lex("vr"),
        &[Token {
            token: TokenType::Keyword,
            value: "vr".to_string(),
            start: 0,
            end: 1,
        }]
    );
}
