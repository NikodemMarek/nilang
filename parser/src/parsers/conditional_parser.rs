use errors::NilangError;
use nilang_types::{
    nodes::{Conditional, StatementNode},
    tokens::{Keyword, TokenType},
};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::parse_statement};

use super::parse_expression;

pub fn parse_conditional<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::If)?;

    let condition = parse_expression(tokens)?;

    tokens.assume(TokenType::OpeningBrace)?;

    // TODO: Extract to scope parser
    let mut body = Vec::new();
    while tokens.peek_valid()?.token != TokenType::ClosingBrace {
        body.push(parse_statement(tokens)?);
    }

    tokens.assume(TokenType::ClosingBrace)?;

    Ok(StatementNode::Conditional(Conditional {
        condition,
        body: body.into(),
    }))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Conditional, ExpressionNode, FunctionCall, StatementNode},
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

    #[test]
    fn test_parse_conditional_with_body() {
        assert_eq!(
            parse_conditional(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::If),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("false".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
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
                condition: ExpressionNode::Boolean(false),
                body: Box::new([StatementNode::FunctionCall(FunctionCall {
                    name: "test".into(),
                    arguments: [].into()
                })])
            })
        );
    }
}
