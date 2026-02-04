use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::statements::Parameter,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_type_annotation;

pub fn parse_parameter_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<[Parameter]>, NilangError> {
    tokens.assume(TokenType::OpeningParenthesis)?;

    let mut parameters = Vec::new();

    loop {
        match tokens.assume_next()? {
            Token {
                token: TokenType::Identifier(value),
                ..
            } => {
                parameters.push((value, parse_type_annotation(tokens)?));

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
            } => break,
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingParenthesis,
                ]))
                .into(),
            })?,
        }
    }

    Ok(parameters.into())
}

#[cfg(test)]
mod test {
    use nilang_types::{
        nodes::Type,
        tokens::{Token, TokenType},
    };

    use crate::{
        multi_peekable::MultiPeekable, parsers::parameter_list_parser::parse_parameter_list,
    };

    #[test]
    fn test_parameter_list() {
        assert_eq!(
            parse_parameter_list(&mut MultiPeekable::new(
                [
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
            ),)
            .unwrap(),
            [("test1".into(), Type::Int), ("test2".into(), Type::Int),].into()
        );
    }
}
