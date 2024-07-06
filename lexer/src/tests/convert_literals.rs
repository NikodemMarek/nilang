use crate::{
    convert,
    tokens::{Token, TokenType},
};

#[test]
fn convert_keywords() {
    assert_eq!(
        convert("main"),
        vec![Token {
            token: TokenType::Literal,
            value: "main".to_string(),
            start: 0,
            end: 3,
        }]
    );
    assert_eq!(
        convert("fn8"),
        vec![Token {
            token: TokenType::Literal,
            value: "fn8".to_string(),
            start: 0,
            end: 2,
        }]
    );
    assert_eq!(
        convert("_rt"),
        vec![Token {
            token: TokenType::Literal,
            value: "_rt".to_string(),
            start: 0,
            end: 2,
        }]
    );
    assert_eq!(
        convert("v33ry__C0mpL3x"),
        vec![Token {
            token: TokenType::Literal,
            value: "v33ry__C0mpL3x".to_string(),
            start: 0,
            end: 13,
        }]
    );
    assert_eq!(
        convert("ClassName"),
        vec![Token {
            token: TokenType::Literal,
            value: "ClassName".to_string(),
            start: 0,
            end: 8,
        }]
    );
}
