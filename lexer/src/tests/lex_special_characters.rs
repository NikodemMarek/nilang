use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_special_characters() {
    assert_eq!(
        &lex(" (5)"),
        &[
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: 3,
                end: 3,
            },
        ]
    );
    assert_eq!(
        &lex("(5 + 4)"),
        &[
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 1,
                end: 1,
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 3,
                end: 3,
            },
            Token {
                token: TokenType::Number,
                value: "4".to_string(),
                start: 5,
                end: 5,
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: 6,
                end: 6,
            },
        ]
    );
    assert_eq!(
        &lex("a = b"),
        &[
            Token {
                token: TokenType::Literal,
                value: "a".to_string(),
                start: 0,
                end: 0,
            },
            Token {
                token: TokenType::Equals,
                value: "=".to_string(),
                start: 2,
                end: 2,
            },
            Token {
                token: TokenType::Literal,
                value: "b".to_string(),
                start: 4,
                end: 4,
            },
        ]
    );
}
