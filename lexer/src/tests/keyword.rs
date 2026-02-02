use nilang_types::tokens::{Keyword, Token, TokenType};

use crate::lex;

#[test]
fn keyword() {
    assert_eq!(
        lex("fn").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword(Keyword::Function),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("rt").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword(Keyword::Return),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("vr").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword(Keyword::Variable),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("if").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword(Keyword::If),
            start: (0, 0),
            end: (0, 1),
        }
    );

    assert_eq!(
        lex("el").next().unwrap().unwrap(),
        Token {
            token: TokenType::Keyword(Keyword::Else),
            start: (0, 0),
            end: (0, 1),
        }
    );
}
