use std::collections::HashMap;

use errors::ParserErrors;
use nilang_types::tokens::{Token, TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_type_annotation;

pub fn parse_parameter_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<HashMap<Box<str>, Box<str>>, ParserErrors> {
    tokens.assume_opening_parenthesis()?;

    let mut parameters = HashMap::new();

    loop {
        match tokens.assume_next()? {
            Token {
                token: TokenType::Identifier(value),
                ..
            } => {
                parameters.insert(value, parse_type_annotation(tokens)?);

                match tokens.assume_next()? {
                    Token {
                        token: TokenType::ClosingParenthesis,
                        ..
                    } => break,
                    Token {
                        token: TokenType::Comma,
                        ..
                    } => {}
                    Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                        loc: start,
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingParenthesis,
                ..
            } => break,
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingParenthesis,
                ]),
                loc: start,
            })?,
        }
    }

    Ok(parameters)
}

#[cfg(test)]
mod test {
    use nilang_types::tokens::{Token, TokenType};

    use crate::parsers::parameter_list_parser::parse_parameter_list;

    #[test]
    fn test_parameter_list() {
        assert_eq!(
            parse_parameter_list(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test1".into()),
                        start: (0, 1),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 7),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (0, 11),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 12),
                        end: (0, 16),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 17),
                        end: (0, 17),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 18),
                        end: (0, 20),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 21),
                        end: (0, 21),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            [
                ("test1".into(), "int".into()),
                ("test2".into(), "int".into()),
            ]
            .into()
        );
    }
}
