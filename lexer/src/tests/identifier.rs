use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn identifier() {
    assert_eq!(
        lex("main").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier,
            value: "main".to_string(),
            start: (0, 0),
            end: (0, 3),
        }
    );

    assert_eq!(
        lex("fn8").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier,
            value: "fn8".to_string(),
            start: (0, 0),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex("_rt").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier,
            value: "_rt".to_string(),
            start: (0, 0),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex("v33ry__C0mpL3x").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier,
            value: "v33ry__C0mpL3x".to_string(),
            start: (0, 0),
            end: (0, 13),
        }
    );

    assert_eq!(
        lex("ClassName").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier,
            value: "ClassName".to_string(),
            start: (0, 0),
            end: (0, 8),
        }
    );
}
