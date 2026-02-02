use errors::NilangError;
use nilang_types::{
    nodes::{Conditional, StatementNode},
    tokens::Keyword,
};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::scope_parser::parse_scope};

use super::parse_expression;

pub fn parse_conditional<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::If)?;

    let condition = parse_expression(tokens)?;
    let body = parse_scope(tokens)?;

    Ok(StatementNode::Conditional(Conditional { condition, body }))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Conditional, ExpressionNode, StatementNode},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::parsers::conditional_parser::parse_conditional;

    #[test]
    fn test_parse_simple_conditional() {
        assert_eq!(
            parse_conditional(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::If),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("true".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            StatementNode::Conditional(Conditional {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([])
            })
        );
    }
}
