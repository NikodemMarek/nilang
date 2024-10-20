use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows_no_rearrange,
};

pub fn parse_parenthesis<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    let start = if let Some(Ok(Token {
        token: TokenType::OpeningParenthesis,
        start,
        ..
    })) = tokens.next()
    {
        start
    } else {
        unreachable!()
    };

    let content = match tokens.peek() {
        Some(Ok(Token {
            token: TokenType::Literal(_),
            ..
        })) => {
            let literal = parse_literal(tokens);
            parse_operation_if_operator_follows_no_rearrange(tokens, literal?)?
        }
        Some(Ok(Token {
            token: TokenType::Identifier(_),
            ..
        })) => {
            let identifier = parse_identifier(tokens);
            parse_operation_if_operator_follows_no_rearrange(tokens, identifier?)?
        }
        Some(Ok(Token {
            token: TokenType::OpeningParenthesis,
            ..
        })) => {
            let parenthesis = parse_parenthesis(tokens);
            parse_operation_if_operator_follows_no_rearrange(tokens, parenthesis?)?
        }
        Some(Ok(Token {
            token: TokenType::ClosingParenthesis,
            end,
            ..
        })) => Err(ParserErrors::EmptyParenthesis {
            from: start,
            to: *end,
        })?,
        Some(Ok(Token { token, start, .. })) => Err(ParserErrors::UnexpectedToken {
            token: token.clone(),
            loc: *start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    match tokens.next() {
        Some(Ok(Token {
            token: TokenType::ClosingParenthesis,
            ..
        })) => {}
        Some(Ok(Token { start, .. })) => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::ClosingParenthesis]),
            loc: start,
        })?,
        Some(Err(e)) => Err(ParserErrors::LexerError(e))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    };

    Ok(content)
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::parenthesis_parser::parse_parenthesis;

    #[test]
    fn test_parse_parenthesis() {
        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("1".into()),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(4.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(1.)),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("1".into()),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Multiply),
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("2".into()),
                        start: (0, 11),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 12),
                        end: (0, 12),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(4.)),
                        b: Box::new(Node::Number(9.)),
                    }),
                    b: Box::new(Node::Number(1.)),
                }),
                b: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(2.)),
                }),
            }
        );
    }
}
