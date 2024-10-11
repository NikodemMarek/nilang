use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_keywords() {
    assert_eq!(
        &lex("main").unwrap(),
        &[Token {
            token: TokenType::Literal,
            value: "main".to_string(),
            start: (0, 0),
            end: (0, 3),
        }]
    );

    assert_eq!(
        &lex("fn8").unwrap(),
        &[Token {
            token: TokenType::Literal,
            value: "fn8".to_string(),
            start: (0, 0),
            end: (0, 2),
        }]
    );

    assert_eq!(
        &lex("_rt").unwrap(),
        &[Token {
            token: TokenType::Literal,
            value: "_rt".to_string(),
            start: (0, 0),
            end: (0, 2),
        }]
    );

    assert_eq!(
        &lex("v33ry__C0mpL3x").unwrap(),
        &[Token {
            token: TokenType::Literal,
            value: "v33ry__C0mpL3x".to_string(),
            start: (0, 0),
            end: (0, 13),
        }]
    );

    assert_eq!(
        &lex("ClassName").unwrap(),
        &[Token {
            token: TokenType::Literal,
            value: "ClassName".to_string(),
            start: (0, 0),
            end: (0, 8),
        }]
    );
}
