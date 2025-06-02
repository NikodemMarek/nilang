use nilang_types::tokens::TokenType;

use crate::lex;

#[test]
fn literal() {
    assert_eq!(
        *lex("5  ").next().unwrap().unwrap(),
        TokenType::Literal("5".into()),
    );

    assert_eq!(
        *lex("4.  ").next().unwrap().unwrap(),
        TokenType::Literal("4.".into()),
    );

    assert_eq!(
        *lex(".9").next().unwrap().unwrap(),
        TokenType::Literal(".9".into()),
    );

    assert_eq!(
        *lex("3.7").next().unwrap().unwrap(),
        TokenType::Literal("3.7".into()),
    );
}
