use errors::NilangError;
use nilang_types::{nodes::StatementNode, tokens::TokenType};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::parse_statement};

pub fn parse_scope<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<[StatementNode]>, NilangError> {
    tokens.assume(TokenType::OpeningBrace)?;

    let mut body = Vec::new();
    while tokens.peek_valid()?.token != TokenType::ClosingBrace {
        body.push(parse_statement(tokens)?);
    }

    tokens.assume(TokenType::ClosingBrace)?;

    Ok(body.into())
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{FunctionCall, StatementNode},
        tokens::{Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::scope_parser::parse_scope};

    #[test]
    fn test_parse_empty_scope() {
        assert_eq!(
            parse_scope(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            [].into()
        );
    }

    #[test]
    fn test_parse_scope_with_statement() {
        assert_eq!(
            parse_scope(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            [StatementNode::FunctionCall(FunctionCall {
                name: "test".into(),
                arguments: [].into()
            })]
            .into()
        );
    }
}
