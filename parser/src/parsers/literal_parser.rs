use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::nodes::ExpressionNode;

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_literal<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, NilangError> {
    let (start, end, value) = tokens.assume_literal()?;

    if value.starts_with('\'') && value.ends_with('\'') {
        if value.len() != 3 {
            return Err(NilangError {
                location: CodeLocation::range(start.0, start.1, end.0, end.1),
                error: ParserErrors::InvalidLiteral.into(),
            });
        }
        return Ok(ExpressionNode::Char(value.chars().nth(1).unwrap()));
    }

    if value.starts_with('"') && value.ends_with('"') {
        return Ok(ExpressionNode::String(value[1..value.len() - 1].into()));
    }

    if &*value == "true" {
        return Ok(ExpressionNode::Boolean(true));
    }
    if &*value == "false" {
        return Ok(ExpressionNode::Boolean(false));
    }

    Ok(match value.parse() {
        Ok(parsed) => ExpressionNode::Number(parsed),
        Err(_) => Err(NilangError {
            location: CodeLocation::range(start.0, start.1, end.0, end.1),
            error: ParserErrors::InvalidLiteral.into(),
        })?,
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::ExpressionNode,
        tokens::{Token, TokenType},
    };

    use crate::parsers::literal_parser::parse_literal;

    #[test]
    fn test_parse_booleans() {
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("true".into()),
                    start: (0, 0),
                    end: (0, 3),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Boolean(true)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("false".into()),
                    start: (0, 0),
                    end: (0, 4),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Boolean(false)
        )
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("54".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Number(54.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("6.".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Number(6.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal(".2".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Number(0.2)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("8.5".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::Number(8.5)
        );
    }
}
