use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

use super::parse;

pub fn parse_function_declaration<'a, I>(
    tokens: &mut Peekable<I>,
    Token {
        token: _,
        value: _,
        start: _,
        end,
    }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    Node::FunctionDeclaration {
        name: match tokens.next() {
            Some(Token {
                token: TokenType::Literal,
                value,
                ..
            }) => value.to_owned(),
            _ => panic!("[{}] Expected a function name", end + 1),
        },
        parameters: if let (
            Some(Token {
                token: TokenType::OpeningParenthesis,
                ..
            }),
            Some(Token {
                token: TokenType::ClosingParenthesis,
                ..
            }),
        ) = (tokens.next(), tokens.next())
        {
            Vec::new()
        } else {
            todo!()
        },
        body: Box::new({
            if let scope @ Node::Scope(_) = parse(&mut Vec::new(), tokens) {
                scope
            } else {
                panic!("[{}] Expected a scope", end + 1)
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{nodes::Node, parse};

    #[test]
    fn parse_function_declaration() {
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
}
