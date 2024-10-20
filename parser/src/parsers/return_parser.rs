use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows, parenthesis_parser::parse_parenthesis,
};

pub fn parse_return<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Keyword,
            value,
            ..
        })) => {
            if value != "rt" {
                Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([TokenType::Keyword]),
                    loc: (0, 1),
                })?
            }
        }
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Keyword]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    let value = match tokens.peek() {
        Some(Ok(Token {
            token: TokenType::Literal,
            ..
        })) => {
            let literal = parse_literal(tokens);
            parse_operation_if_operator_follows(tokens, literal?)?
        }
        Some(Ok(Token {
            token: TokenType::Identifier,
            ..
        })) => {
            let identifier = parse_identifier(tokens);
            parse_operation_if_operator_follows(tokens, identifier?)?
        }
        Some(Ok(Token {
            token: TokenType::OpeningParenthesis,
            ..
        })) => {
            let parenthesis = parse_parenthesis(tokens);
            parse_operation_if_operator_follows(tokens, parenthesis?)?
        }
        Some(Ok(Token { end, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([
                TokenType::Literal,
                TokenType::Identifier,
                TokenType::OpeningParenthesis,
            ]),
            loc: *end,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::Semicolon,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Semicolon]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    Ok(Node::Return(Box::new(value)))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::return_parser::parse_return;

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        value: "(".to_string(),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Operator,
                        value: "+".to_string(),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "9".to_string(),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        value: ")".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator,
                        value: "+".to_string(),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "9".to_string(),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Operator,
                        value: "+".to_string(),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "9".to_string(),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator,
                        value: "+".to_string(),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "5".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
