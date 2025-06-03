use errors::NilangError;
use nilang_types::{nodes::Arguments, tokens::TokenType, Localizable as L, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_arguments<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<Arguments>, NilangError> {
    let start = tokens.assume(TokenType::OpeningParenthesis)?;

    let mut arguments = Vec::new();
    while let L {
        object: TokenType::Identifier(_) | TokenType::Literal(_) | TokenType::OpeningParenthesis,
        ..
    } = tokens.peek_valid()?
    {
        arguments.push(parse_expression(tokens)?);

        if let L {
            object: TokenType::Comma,
            ..
        } = tokens.peek_valid()?
        {
            tokens.assume_next()?;
        } else {
            break;
        }
    }

    let end = tokens.assume(TokenType::ClosingParenthesis)?;

    Ok(L::new(Location::between(&start, &end), arguments.into()))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
        tokens::TokenType,
        Localizable as L,
    };

    use crate::parsers::arguments_parser::parse_arguments;

    #[test]
    fn test_parse_argument_list() {
        assert_eq!(
            parse_arguments(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(L::irrelevant(TokenType::Comma,)),
                    Ok(L::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            [
                L::irrelevant(ExpressionNode::Number(5.)),
                L::irrelevant(ExpressionNode::VariableReference("x".into()))
            ]
            .into()
        );

        assert_eq!(
            parse_arguments(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("4".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            [L::irrelevant(ExpressionNode::Operation {
                operator: L::irrelevant(Operator::Add),
                a: Box::new(L::irrelevant(ExpressionNode::VariableReference("x".into()))),
                b: Box::new(L::irrelevant(ExpressionNode::Number(4.))),
            })]
            .into()
        );
    }
}
