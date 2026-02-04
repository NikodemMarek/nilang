use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::expressions::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_argument_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<[ExpressionNode]>, NilangError> {
    tokens.assume(TokenType::OpeningParenthesis)?;

    let mut arguments = Vec::new();

    loop {
        match tokens.peek_valid()? {
            Token {
                token:
                    TokenType::Literal(_) | TokenType::Identifier(_) | TokenType::OpeningParenthesis,
                ..
            } => {
                arguments.push(parse_expression(tokens)?);

                match tokens.assume_next()? {
                    Token {
                        token: TokenType::ClosingParenthesis,
                        ..
                    } => break,
                    Token {
                        token: TokenType::Comma,
                        ..
                    } => {}
                    Token { start, .. } => Err(NilangError {
                        location: CodeLocation::at(start.0, start.1),
                        error: ParserErrors::ExpectedTokens(Vec::from([
                            TokenType::Comma,
                            TokenType::ClosingParenthesis,
                        ]))
                        .into(),
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingParenthesis,
                ..
            } => {
                tokens.next();
                break;
            }
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
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

    Ok(arguments.into())
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::expressions::ExpressionNode,
        tokens::{Token, TokenType},
    };

    use crate::{
        multi_peekable::MultiPeekable, parsers::argument_list_parser::parse_argument_list,
    };

    #[test]
    fn test_parse_argument_list() {
        assert_eq!(
            parse_argument_list(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
            ))
            .unwrap(),
            [
                ExpressionNode::Number(5.),
                ExpressionNode::VariableReference("x".into())
            ]
            .into()
        );
    }
}
