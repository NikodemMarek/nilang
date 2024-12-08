use errors::ParserErrors;
use nilang_types::{nodes::Node, tokens::Keyword};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::value_yielding_parser::parse_value_yielding;

pub fn parse_return<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    tokens.assume_keyword(Keyword::Return)?;

    let value = parse_value_yielding(tokens)?;

    tokens.assume_semicolon()?;

    Ok(Node::Return(Box::new(value)))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::parsers::return_parser::parse_return;

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::Return(Box::new(Node::Number(6.)))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::Return(Box::new(Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }))
        );
    }
}
