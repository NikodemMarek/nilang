use crate::{
    convert,
    tokens::{Token, TokenType},
};

#[test]
fn convert_numbers() {
    assert_eq!(
        convert("5  "),
        vec![Token {
            token: TokenType::Number,
            value: "5".to_string(),
            start: 0,
            end: 0,
        }]
    );
    assert_eq!(
        convert("4.  "),
        vec![Token {
            token: TokenType::Number,
            value: "4.".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        convert(".9"),
        vec![Token {
            token: TokenType::Number,
            value: ".9".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        convert("3.7"),
        vec![Token {
            token: TokenType::Number,
            value: "3.7".to_string(),
            start: 0,
            end: 2,
        }]
    );
}
