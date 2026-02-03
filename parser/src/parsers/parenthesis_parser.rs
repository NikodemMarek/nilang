use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows_no_rearrange,
};

pub fn parse_parenthesis<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, NilangError> {
    let start = tokens.assume(TokenType::OpeningParenthesis)?;

    let content = match tokens.peek_valid()? {
        Token {
            token: TokenType::Literal(_),
            ..
        } => {
            let literal = parse_literal(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, literal)?
        }
        Token {
            token: TokenType::Identifier(_),
            ..
        } => {
            let identifier = parse_identifier(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, identifier)?
        }
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => {
            let parenthesis = parse_parenthesis(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, parenthesis)?
        }
        Token {
            token: TokenType::ClosingParenthesis,
            end,
            ..
        } => Err(NilangError {
            location: CodeLocation::range(start.0, start.1, end.0, end.1),
            error: ParserErrors::EmptyParenthesis.into(),
        })?,
        Token { token, start, .. } => Err(NilangError {
            location: CodeLocation::at(start.0, start.1),
            error: ParserErrors::UnexpectedToken(token.clone()).into(),
        })?,
    };

    tokens.assume(TokenType::ClosingParenthesis)?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
        tokens::{Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::parenthesis_parser::parse_parenthesis};

    #[test]
    fn test_parse_parenthesis() {
        assert_eq!(
            parse_parenthesis(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Number(6.)),
                b: Box::new(ExpressionNode::Number(9.)),
            }
        );

        assert_eq!(
            parse_parenthesis(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Number(6.)),
                b: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(9.)),
                    b: Box::new(ExpressionNode::Number(5.)),
                }),
            }
        );

        assert_eq!(
            parse_parenthesis(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Multiply,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Add,
                    a: Box::new(ExpressionNode::Number(4.)),
                    b: Box::new(ExpressionNode::Number(9.)),
                }),
                b: Box::new(ExpressionNode::Number(1.)),
            }
        );

        assert_eq!(
            parse_parenthesis(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            ExpressionNode::Operation {
                operator: Operator::Add,
                a: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Operation {
                        operator: Operator::Add,
                        a: Box::new(ExpressionNode::Number(4.)),
                        b: Box::new(ExpressionNode::Number(9.)),
                    }),
                    b: Box::new(ExpressionNode::Number(1.)),
                }),
                b: Box::new(ExpressionNode::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(ExpressionNode::Number(6.)),
                    b: Box::new(ExpressionNode::Number(2.)),
                }),
            }
        );
    }
}
