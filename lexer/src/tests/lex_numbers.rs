use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_numbers() {
    assert_eq!(
        &lex("5  ").unwrap(),
        &[Token {
            token: TokenType::Number,
            value: "5".to_string(),
            start: (0, 0),
            end: (0, 0),
        }]
    );

    assert_eq!(
        &lex("4.  ").unwrap(),
        &[Token {
            token: TokenType::Number,
            value: "4.".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex(".9").unwrap(),
        &[Token {
            token: TokenType::Number,
            value: ".9".to_string(),
            start: (0, 0),
            end: (0, 1),
        }]
    );

    assert_eq!(
        &lex("3.7").unwrap(),
        &[Token {
            token: TokenType::Number,
            value: "3.7".to_string(),
            start: (0, 0),
            end: (0, 2),
        }]
    );
}
