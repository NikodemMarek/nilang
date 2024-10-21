use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn keyword() {
    assert_eq!(
        lex("fn").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword("fn".into()),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("rt").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword("rt".into()),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("vr").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword("vr".into()),
            start: (0, 0),
            end: (0, 1),
        }
    );
}
