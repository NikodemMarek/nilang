use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn identifier() {
    assert_eq!(
        lex("main").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("main".into()),
            start: (0, 0),
            end: (0, 3),
        }
    );

    assert_eq!(
        lex("fn8").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("fn8".into()),
            start: (0, 0),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex("_rt").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("_rt".into()),
            start: (0, 0),
            end: (0, 2),
        }
    );

    assert_eq!(
        lex("v33ry__C0mpL3x").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("v33ry__C0mpL3x".into()),
            start: (0, 0),
            end: (0, 13),
        }
    );

    assert_eq!(
        lex("ClassName").next().unwrap().unwrap(),
        Token {
            token: TokenType::Identifier("ClassName".into()),
            start: (0, 0),
            end: (0, 8),
        }
    );
}
