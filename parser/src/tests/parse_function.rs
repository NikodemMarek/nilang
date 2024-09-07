use crate::{nodes::Node, parse};
use nilang_lexer::tokens::{Token, TokenType};

#[test]
fn parse_function() {
    assert_eq!(
        &parse(&[
            Token {
                token: TokenType::Keyword,
                value: "fn".to_string(),
                start: 0,
                end: 1,
            },
            Token {
                token: TokenType::Literal,
                value: "main".to_string(),
                start: 3,
                end: 6,
            },
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: 7,
                end: 7,
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: 8,
                end: 8,
            },
            Token {
                token: TokenType::OpeningBrace,
                value: "{".to_string(),
                start: 9,
                end: 9,
            },
            Token {
                token: TokenType::Keyword,
                value: "rt".to_string(),
                start: 11,
                end: 12,
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: 14,
                end: 14,
            },
            Token {
                token: TokenType::ClosingBrace,
                value: "}".to_string(),
                start: 15,
                end: 15,
            },
        ]),
        &[Node::FunctionDeclaration {
            name: "main".to_string(),
            parameters: Vec::new(),
            body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                Node::Number(6.)
            ))]))),
        }]
    );
}
