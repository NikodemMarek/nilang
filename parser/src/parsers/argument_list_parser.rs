use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::value_yielding_parser::parse_value_yielding;

pub fn parse_argument_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<[Node]>, ParserErrors> {
    tokens.assume_opening_parenthesis()?;

    let mut arguments = Vec::new();

    loop {
        match tokens.peek_valid()? {
            Token {
                token:
                    TokenType::Literal(_) | TokenType::Identifier(_) | TokenType::OpeningParenthesis,
                ..
            } => {
                arguments.push(parse_value_yielding(tokens)?);

                match tokens.assume_next()? {
                    Token {
                        token: TokenType::ClosingParenthesis,
                        ..
                    } => break,
                    Token {
                        token: TokenType::Comma,
                        ..
                    } => {}
                    Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                        loc: start,
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingParenthesis,
                ..
            } => {
                tokens.next();
                break;
            }
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::Literal("".into()),
                    TokenType::OpeningParenthesis,
                    TokenType::ClosingParenthesis,
                ]),
                loc: *start,
            })?,
        }
    }

    Ok(arguments.into())
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::argument_list_parser::parse_argument_list;

    #[test]
    fn test_parse_argument_list() {
        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
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
            [Node::Number(5.), Node::VariableReference("x".into())].into()
        );

        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
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
            [Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::VariableReference("x".into())),
                b: Box::new(Node::Number(4.)),
            }]
            .into()
        );
    }
}
