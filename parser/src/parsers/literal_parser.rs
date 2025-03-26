use errors::ParserErrors;
use nilang_types::nodes::ExpressionNode;

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_literal<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, ParserErrors> {
    let (start, end, value) = tokens.assume_literal()?;

    Ok(match value.parse() {
        Ok(parsed) => ExpressionNode::Number(parsed),
        Err(_) => Err(ParserErrors::NotANumber {
            from: start,
            to: end,
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
