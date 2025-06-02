use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_argument_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<Box<[Localizable<ExpressionNode>]>>, NilangError> {
    let start = tokens.assume(TokenType::OpeningParenthesis)?;

    let mut arguments = Vec::new();

    loop {
        match tokens.peek_valid()? {
            Localizable {
                object:
                    TokenType::Literal(_) | TokenType::Identifier(_) | TokenType::OpeningParenthesis,
                ..
            } => {
                arguments.push(parse_expression(tokens)?);

                match tokens.assume_next()? {
                    Localizable {
                        object: TokenType::ClosingParenthesis,
                        location: end,
                    } => {
                        return Ok(Localizable::new(
                            Location::between(&start, &end),
                            arguments.into(),
                        ));
                    }
                    Localizable {
                        object: TokenType::Comma,
                        ..
                    } => {}
                    Localizable { location, .. } => Err(NilangError {
                        location,
                        error: ParserErrors::ExpectedTokens(Vec::from([
                            TokenType::Comma,
                            TokenType::ClosingParenthesis,
                        ]))
                        .into(),
                    })?,
                }
            }
            Localizable {
                object: TokenType::ClosingParenthesis,
                ..
            } => {
                let end = tokens.assume(TokenType::ClosingParenthesis)?;
                return Ok(Localizable::new(
                    Location::between(&start, &end),
                    arguments.into(),
                ));
            }
            Localizable { location, .. } => Err(NilangError {
                location: *location,
                error: ParserErrors::ExpectedTokens(Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::Literal("".into()),
                    TokenType::OpeningParenthesis,
                    TokenType::ClosingParenthesis,
                ]))
                .into(),
            })?,
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, Operator},
        tokens::TokenType,
        Localizable,
    };

    use crate::parsers::argument_list_parser::parse_argument_list;

    #[test]
    fn test_parse_argument_list() {
        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Literal("5".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Comma,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            [
                Localizable::irrelevant(ExpressionNode::Number(5.)),
                Localizable::irrelevant(ExpressionNode::VariableReference(
                    Localizable::irrelevant("x".into())
                ))
            ]
            .into()
        );

        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("4".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            [Localizable::irrelevant(ExpressionNode::Operation {
                operator: Localizable::irrelevant(Operator::Add),
                a: Box::new(Localizable::irrelevant(ExpressionNode::VariableReference(
                    Localizable::irrelevant("x".into())
                ))),
                b: Box::new(Localizable::irrelevant(ExpressionNode::Number(4.))),
            })]
            .into()
        );
    }
}
