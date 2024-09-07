use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, parse};

#[test]
fn parse_variable() {
    assert_eq!(
        &parse(&[
            Token {
                token: TokenType::Keyword,
                value: "vr".to_string(),
                start: 0,
                end: 1,
            },
            Token {
                token: TokenType::Literal,
                value: "test".to_string(),
                start: 1,
                end: 4,
            },
            Token {
                token: TokenType::Equals,
                value: "=".to_string(),
                start: 5,
                end: 5,
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: 6,
                end: 6,
            },
        ]),
        &[Node::VariableDeclaration {
            name: "test".to_string(),
            value: Box::new(Node::Number(9.))
        }]
    );
}
