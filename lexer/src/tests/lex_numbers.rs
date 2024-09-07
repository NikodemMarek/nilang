use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_numbers() {
    assert_eq!(
        &lex("5  "),
        &[Token {
            token: TokenType::Number,
            value: "5".to_string(),
            start: 0,
            end: 0,
        }]
    );
    assert_eq!(
        &lex("4.  "),
        &[Token {
            token: TokenType::Number,
            value: "4.".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        &lex(".9"),
        &[Token {
            token: TokenType::Number,
            value: ".9".to_string(),
            start: 0,
            end: 1,
        }]
    );
    assert_eq!(
        &lex("3.7"),
        &[Token {
            token: TokenType::Number,
            value: "3.7".to_string(),
            start: 0,
            end: 2,
        }]
    );
}
