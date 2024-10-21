use nilang_types::tokens::{Token, TokenType};

use crate::lex;

#[test]
fn literal() {
    assert_eq!(
        lex("5  ").next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("5".into()),
            start: (0, 0),
            end: (0, 0),
        }
    );

    assert_eq!(
        lex("4.  ").next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("4.".into()),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex(".9").next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal(".9".into()),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("3.7").next().unwrap().unwrap(),
        Token {
            token: TokenType::Literal("3.7".into()),
            start: (0, 0),
            end: (0, 2),
        }
    );
}
