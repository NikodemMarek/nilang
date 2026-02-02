use errors::NilangError;
use nilang_types::{
    nodes::{Conditional, ExpressionNode, StatementNode},
    tokens::{Keyword, TokenType},
};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::scope_parser::parse_scope};

use super::parse_expression;

pub fn parse_conditional<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::If)?;

    let condition = parse_expression(tokens)?;
    let body = parse_scope(tokens)?;

    let chained = if tokens.peek_valid()?.token == TokenType::Keyword(Keyword::Else) {
        tokens.assume_keyword(Keyword::Else)?;
        Some(Box::new(Conditional {
            condition: ExpressionNode::Boolean(true),
            body: parse_scope(tokens)?,
            chained: None,
        }))
    } else {
        None
    };

    Ok(StatementNode::Conditional(Conditional {
        condition,
        body,
        chained,
    }))
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
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            StatementNode::Conditional(Conditional {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
                chained: None
            })
        );
    }

    #[test]
    fn test_parse_conditional_with_else() {
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
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Else),
                        start: (0, 6),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            StatementNode::Conditional(Conditional {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
                chained: Some(Box::new(Conditional {
                    condition: ExpressionNode::Boolean(true),
                    body: Box::new([]),
                    chained: None
                }))
            })
        );
    }
}
