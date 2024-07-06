use crate::{
    convert,
    tokens::{Token, TokenType},
};

#[test]
fn convert_keywords() {
    assert_eq!(
        convert("fn"),
        vec![Token {
            token: TokenType::Keyword,
            value: "fn".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        convert("rt"),
        vec![Token {
            token: TokenType::Keyword,
            value: "rt".to_string(),
            start: 0,
            end: 1,
        }]
    );
}
