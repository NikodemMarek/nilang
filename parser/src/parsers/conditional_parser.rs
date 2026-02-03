use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::{Conditional, ExpressionNode},
    tokens::{Keyword, Token, TokenType},
};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::scope_parser::parse_scope};

use super::parse_expression;

pub fn parse_conditional<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Conditional, NilangError> {
    match tokens.peek_valid()? {
        Token {
            token: TokenType::Keyword(Keyword::If),
            ..
        } => parse_if(tokens),
        Token {
            token: TokenType::Keyword(Keyword::ElseIf),
            ..
        } => parse_else_if(tokens),
        Token {
            token: TokenType::Keyword(Keyword::Else),
            ..
        } => parse_else(tokens),
        Token { start, end, .. } => Err(NilangError {
            location: CodeLocation::range(start.0, start.1, end.0, end.1),
            error: ParserErrors::ExpectedTokens(
                [
                    TokenType::Keyword(Keyword::If),
                    TokenType::Keyword(Keyword::Else),
                ]
                .to_vec(),
            )
            .into(),
        }),
    }
}

pub fn parse_if<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Conditional, NilangError> {
    tokens.assume_keyword(Keyword::If)?;

    let condition = parse_expression(tokens)?;
    let body = parse_scope(tokens)?;
    let chained = parse_conditional(tokens).ok().map(Box::new);

    Ok(Conditional {
        condition,
        body,
        chained,
    })
}

pub fn parse_else_if<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Conditional, NilangError> {
    tokens.assume_keyword(Keyword::ElseIf)?;

    let condition = parse_expression(tokens)?;
    let body = parse_scope(tokens)?;
    let chained = parse_conditional(tokens).ok().map(Box::new);

    Ok(Conditional {
        condition,
        body,
        chained,
    })
}

pub fn parse_else<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Conditional, NilangError> {
    tokens.assume_keyword(Keyword::Else)?;

    let body = parse_scope(tokens)?;

    Ok(Conditional {
        condition: ExpressionNode::Boolean(true),
        body,
        chained: None,
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Conditional, ExpressionNode},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::conditional_parser::parse_conditional};

    #[test]
    fn test_parse_simple_conditional() {
        assert_eq!(
            parse_conditional(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            Conditional {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
                chained: None
            }
        );
    }

    #[test]
    fn test_parse_conditional_with_else() {
        assert_eq!(
            parse_conditional(&mut MultiPeekable::new(
                [
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
            ),)
            .unwrap(),
            Conditional {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
                chained: Some(Box::new(Conditional {
                    condition: ExpressionNode::Boolean(true),
                    body: Box::new([]),
                    chained: None
                }))
            }
        );
    }

    #[test]
    fn test_parse_conditional_with_else_if() {
        assert_eq!(
            parse_conditional(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::If),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("false".into()),
                        start: (0, 3),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::ElseIf),
                        start: (0, 7),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("false".into()),
                        start: (0, 9),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            Conditional {
                condition: ExpressionNode::Boolean(false),
                body: Box::new([]),
                chained: Some(Box::new(Conditional {
                    condition: ExpressionNode::Boolean(false),
                    body: Box::new([]),
                    chained: None
                }))
            }
        );
    }

    #[test]
    fn test_parse_conditional_with_else_if_and_else() {
        assert_eq!(
            parse_conditional(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::If),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("false".into()),
                        start: (0, 3),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::ElseIf),
                        start: (0, 7),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("false".into()),
                        start: (0, 9),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Else),
                        start: (0, 16),
                        end: (0, 16),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 17),
                        end: (0, 17),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 18),
                        end: (0, 18),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            Conditional {
                condition: ExpressionNode::Boolean(false),
                body: Box::new([]),
                chained: Some(Box::new(Conditional {
                    condition: ExpressionNode::Boolean(false),
                    body: Box::new([]),
                    chained: Some(Box::new(Conditional {
                        condition: ExpressionNode::Boolean(true),
                        body: Box::new([]),
                        chained: None
                    }))
                }))
            }
        );
    }
}
