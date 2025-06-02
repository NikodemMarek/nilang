use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::ExpressionNode, Localizable};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_literal<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<ExpressionNode>, NilangError> {
    let value = tokens.assume_literal()?;

    if value.starts_with('\'') && value.ends_with('\'') {
        if value.len() != 3 {
            return Err(NilangError {
                location: value.location,
                error: ParserErrors::InvalidLiteral.into(),
            });
        }
        return Ok(Localizable::new(
            value.location,
            ExpressionNode::Char(value.chars().nth(1).unwrap()),
        ));
    }

    if value.starts_with('"') && value.ends_with('"') {
        return Ok(Localizable::new(
            value.location,
            ExpressionNode::String(value[1..value.len() - 1].into()),
        ));
    }

    Ok(match value.parse() {
        Ok(parsed) => Localizable::new(value.location, ExpressionNode::Number(parsed)),
        Err(_) => Err(NilangError {
            location: value.location,
            error: ParserErrors::InvalidLiteral.into(),
        })?,
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable};

    use crate::parsers::literal_parser::parse_literal;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(
            parse_literal(
                &mut [Ok(
                    Localizable::irrelevant(TokenType::Literal("54".into()),)
                )]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Number(54.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(
                    Localizable::irrelevant(TokenType::Literal("6.".into()),)
                )]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Number(6.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(
                    Localizable::irrelevant(TokenType::Literal(".2".into()),)
                )]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Number(0.2)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Localizable::irrelevant(TokenType::Literal(
                    "8.5".into()
                ),))]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::Number(8.5)
        );
    }
}
