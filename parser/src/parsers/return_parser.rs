use errors::NilangError;
use nilang_types::{
    nodes::StatementNode,
    tokens::{Keyword, TokenType},
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_return<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<StatementNode>, NilangError> {
    let start = tokens.assume_keyword(Keyword::Return)?;

    let value = parse_expression(tokens)?;

    let end = tokens.assume(TokenType::Semicolon)?;

    Ok(L::new(
        Location::between(&start, &end),
        StatementNode::Return(Box::new(value)),
    ))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator, StatementNode},
        tokens::{Keyword, TokenType},
        Localizable as L,
    };

    use crate::parsers::return_parser::parse_return;

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(L::irrelevant(TokenType::Literal("6".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(L::irrelevant(ExpressionNode::Number(6.))))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()))),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(L::irrelevant(TokenType::Literal("9".into()))),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis)),
                    Ok(L::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(L::irrelevant(ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
            })))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(L::irrelevant(TokenType::Literal("6".into()))),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(L::irrelevant(TokenType::Literal("9".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(L::irrelevant(ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
            })))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Return),)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(L::irrelevant(ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                })),
                b: Box::new(L::irrelevant(ExpressionNode::Number(5.))),
            })))
        );
    }
}
