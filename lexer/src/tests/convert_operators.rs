use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn convert_operators() {
    assert_eq!(
        lex("  +"),
        vec![Token {
            token: TokenType::Operator,
            value: "+".to_string(),
            start: 2,
            end: 2,
        }]
    );
    assert_eq!(
        lex(" - "),
        vec![Token {
            token: TokenType::Operator,
            value: "-".to_string(),
            start: 1,
            end: 1,
        }]
    );
    assert_eq!(
        lex("*"),
        vec![Token {
            token: TokenType::Operator,
            value: "*".to_string(),
            start: 0,
            end: 0,
        }]
    );
    assert_eq!(
        lex("/"),
        vec![Token {
            token: TokenType::Operator,
            value: "/".to_string(),
            start: 0,
            end: 0,
        }]
    );
    assert_eq!(
        lex("%"),
        vec![Token {
            token: TokenType::Operator,
            value: "%".to_string(),
            start: 0,
            end: 0,
        }]
    );
}
