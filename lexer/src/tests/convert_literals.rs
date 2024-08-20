use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn convert_keywords() {
    assert_eq!(
        lex("main"),
        vec![Token {
            token: TokenType::Literal,
            value: "main".to_string(),
            start: 0,
            end: 3,
        }]
    );
    assert_eq!(
        lex("fn8"),
        vec![Token {
            token: TokenType::Literal,
            value: "fn8".to_string(),
            start: 0,
            end: 2,
        }]
    );
    assert_eq!(
        lex("_rt"),
        vec![Token {
            token: TokenType::Literal,
            value: "_rt".to_string(),
            start: 0,
            end: 2,
        }]
    );
    assert_eq!(
        lex("v33ry__C0mpL3x"),
        vec![Token {
            token: TokenType::Literal,
            value: "v33ry__C0mpL3x".to_string(),
            start: 0,
            end: 13,
        }]
    );
    assert_eq!(
        lex("ClassName"),
        vec![Token {
            token: TokenType::Literal,
            value: "ClassName".to_string(),
            start: 0,
            end: 8,
        }]
    );
}
