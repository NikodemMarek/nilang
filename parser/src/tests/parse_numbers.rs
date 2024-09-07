use crate::{nodes::Node, parse};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_numbers() {
    assert_eq!(
        &parse(&[Token {
            token: TokenType::Number,
            value: "54".to_string(),
            start: 0,
            end: 2,
        }]),
        &[Node::Number(54.)]
    );
    assert_eq!(
        &parse(&[Token {
            token: TokenType::Number,
            value: "6.".to_string(),
            start: 0,
            end: 2,
        }]),
        &[Node::Number(6.)]
    );
    assert_eq!(
        &parse(&[Token {
            token: TokenType::Number,
            value: ".2".to_string(),
            start: 0,
            end: 2,
        }]),
        &[Node::Number(0.2)]
    );
    assert_eq!(
        &parse(&[Token {
            token: TokenType::Number,
            value: "8.5".to_string(),
            start: 0,
            end: 2,
        }]),
        &[Node::Number(8.5)]
    );
}
