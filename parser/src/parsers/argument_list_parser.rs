use errors::ParserErrors;
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_argument_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<[ExpressionNode]>, ParserErrors> {
    tokens.assume(TokenType::OpeningParenthesis)?;

    let mut arguments = Vec::new();

    loop {
        match tokens.peek_valid()? {
            Token {
                token:
                    TokenType::Literal(_) | TokenType::Identifier(_) | TokenType::OpeningParenthesis,
                ..
            } => {
                arguments.push(parse_expression(tokens)?);

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
        nodes::{ExpressionNode, Operator},
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
            [
                ExpressionNode::Number(5.),
                ExpressionNode::VariableReference("x".into())
            ]
            .into()
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
            [ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::VariableReference("x".into())),
                b: Box::new(ExpressionNode::Number(4.)),
            }]
            .into()
        );
    }
}
