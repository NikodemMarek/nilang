use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable as L};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows_no_rearrange,
};

pub fn parse_parenthesis<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<ExpressionNode>, NilangError> {
    let _ = tokens.assume(TokenType::OpeningParenthesis)?;

    let content = match tokens.peek_valid()? {
        L {
            object: TokenType::Literal(_),
            ..
        } => {
            let literal = parse_literal(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, literal)?
        }
        L {
            object: TokenType::Identifier(_),
            ..
        } => {
            let identifier = parse_identifier(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, identifier)?
        }
        L {
            object: TokenType::OpeningParenthesis,
            ..
        } => {
            let parenthesis = parse_parenthesis(tokens)?;
            parse_operation_if_operator_follows_no_rearrange(tokens, parenthesis)?
        }
        L {
            object: TokenType::ClosingParenthesis,
            location,
        } => Err(NilangError {
            location: *location,
            error: ParserErrors::EmptyParenthesis.into(),
        })?,
        L { object, location } => Err(NilangError {
            location: *location,
            error: ParserErrors::UnexpectedToken(object.clone()).into(),
        })?,
    };

    tokens.assume(TokenType::ClosingParenthesis)?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
        tokens::TokenType,
        Localizable as L,
    };

    use crate::parsers::parenthesis_parser::parse_parenthesis;

    #[test]
    fn test_parse_parenthesis() {
        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()))),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(L::irrelevant(TokenType::Literal("9".into()))),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(L::irrelevant(TokenType::Literal("5".into()))),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Multiply),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
                })),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("4".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("1".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Multiply),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(4.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(1.))),
            }
        );

        assert_eq!(
            parse_parenthesis(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("4".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("1".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Multiply),)),
                    Ok(L::irrelevant(TokenType::Literal("2".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Operation {
                        operator: L::irrelevant(Operator::Add),
                        a: Box::new(L::irrelevant(ExpressionNode::Number(4.))),
                        b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                    })),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(1.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Multiply),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(2.))),
                })),
            }
        );
    }
}
