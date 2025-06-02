use errors::NilangError;
use nilang_types::{
    nodes::StatementNode,
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_return<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<StatementNode>, NilangError> {
    let start = tokens.assume_keyword(Keyword::Return)?;

    let value = parse_expression(tokens)?;

    let end = tokens.assume(TokenType::Semicolon)?;

    Ok(Localizable::new(
        Location::between(&start, &end),
        StatementNode::Return(Box::new(value)),
    ))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator, StatementNode},
        tokens::{Keyword, TokenType},
        Localizable,
    };

    use crate::parsers::return_parser::parse_return;

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(Localizable::irrelevant(ExpressionNode::Number(
                6.
            ))))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis)),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()))),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()))),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(Localizable::irrelevant(
                ExpressionNode::Operation {
                    operator: Localizable::irrelevant(Operator::Add),
                    a: Box::new(Localizable::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                }
            )))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(Keyword::Return))),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()))),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add))),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(Localizable::irrelevant(
                ExpressionNode::Operation {
                    operator: Localizable::irrelevant(Operator::Add),
                    a: Box::new(Localizable::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                }
            )))
        );

        assert_eq!(
            parse_return(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(Keyword::Return),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::Return(Box::new(Localizable::irrelevant(
                ExpressionNode::Operation {
                    operator: Localizable::irrelevant(Operator::Add),
                    a: Box::new(Localizable::irrelevant(ExpressionNode::Operation {
                        operator: Localizable::irrelevant(Operator::Add),
                        a: Box::new(Localizable::irrelevant(ExpressionNode::Number(6.))),
                        b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                    })),
                    b: Box::new(Localizable::irrelevant(ExpressionNode::Number(5.))),
                }
            )))
        );
    }
}
