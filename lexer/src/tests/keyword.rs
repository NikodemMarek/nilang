use nilang_types::tokens::{Keyword, TokenType};

use crate::lex;

#[test]
fn keyword() {
    assert_eq!(
        *lex("fn").next().unwrap().unwrap(),
        TokenType::Keyword(Keyword::Function),
    );

    assert_eq!(
        *lex("rt").next().unwrap().unwrap(),
        TokenType::Keyword(Keyword::Return),
    );

    assert_eq!(
        *lex("vr").next().unwrap().unwrap(),
        TokenType::Keyword(Keyword::Variable),
    );
}
