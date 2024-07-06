use crate::{
    convert,
    tokens::{Token, TokenType},
};

#[test]
fn convert_operations() {
    assert_eq!(
        convert("5+4"),
        vec![
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
        convert("5.5 * 8"),
        vec![
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
