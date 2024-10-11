use crate::{
    lex,
    tokens::{Token, TokenType},
};

#[test]
fn lex_special_characters() {
    assert_eq!(
        &lex(" (5)").unwrap(),
        &[
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
        ]
    );

    assert_eq!(
        &lex("(5 + 4)").unwrap(),
        &[
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "4".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ]
    );

    assert_eq!(
        &lex("a = b").unwrap(),
        &[
            Token {
                token: TokenType::Literal,
                value: "a".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Equals,
                value: "=".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Literal,
                value: "b".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ]
    );
}
