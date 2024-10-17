use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn special_character() {
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
                token: TokenType::Literal,
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
                token: TokenType::Literal,
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
                token: TokenType::Literal,
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
                token: TokenType::Identifier,
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
                token: TokenType::Identifier,
                value: "b".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ]
    );
}
