use nilang_types::tokens::TokenType;

use crate::lex;

#[test]
fn identifier() {
    assert_eq!(
        *lex("main").next().unwrap().unwrap(),
        TokenType::Identifier("main".into()),
    );

    assert_eq!(
        *lex("fn8").next().unwrap().unwrap(),
        TokenType::Identifier("fn8".into()),
    );

    assert_eq!(
        *lex("_rt").next().unwrap().unwrap(),
        TokenType::Identifier("_rt".into()),
    );

    assert_eq!(
        *lex("v33ry__C0mpL3x").next().unwrap().unwrap(),
        TokenType::Identifier("v33ry__C0mpL3x".into()),
    );

    assert_eq!(
        *lex("ClassName").next().unwrap().unwrap(),
        TokenType::Identifier("ClassName".into()),
    );
}
