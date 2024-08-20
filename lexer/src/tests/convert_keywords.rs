use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn convert_keywords() {
    assert_eq!(
        lex("fn"),
        vec![Token {
            token: TokenType::Keyword,
            value: "fn".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        lex("rt"),
        vec![Token {
            token: TokenType::Keyword,
            value: "rt".to_string(),
            start: 0,
            end: 1,
        }]
    );
}
