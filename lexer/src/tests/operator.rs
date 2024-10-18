use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn operator() {
    assert_eq!(
        lex("  +").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "+".to_string(),
            start: (0, 2),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex(" - ").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "-".to_string(),
            start: (0, 1),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("*").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "*".to_string(),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("/").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "/".to_string(),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("%").next().unwrap().unwrap(),
        Token {
            token: TokenType::Operator,
            value: "%".to_string(),
            start: (0, 0),
            end: (0, 0),
        }
    );
}
