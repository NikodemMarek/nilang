use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_operations() {
    assert_eq!(
        &lex("5+4"),
        &[
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: "4".to_string(),
                start: 2,
                end: 2,
            },
        ]
    );
    assert_eq!(
        &lex("5.5 * 8"),
        &[
            Token {
                token: TokenType::Number,
                value: "5.5".to_string(),
                start: 0,
                end: 2,
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: 4,
                end: 4,
            },
            Token {
                token: TokenType::Number,
                value: "8".to_string(),
                start: 6,
                end: 6,
            },
        ]
    );
}
