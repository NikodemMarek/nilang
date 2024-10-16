use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn lex_operations() {
    assert_eq!(
        &lex("5+4").unwrap(),
        &[
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Number,
                value: "4".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
        ]
    );

    assert_eq!(
        &lex("5.5 * 8").unwrap(),
        &[
            Token {
                token: TokenType::Number,
                value: "5.5".to_string(),
                start: (0, 0,),
                end: (0, 2),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "8".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ]
    );
}
