use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::Parameters, tokens::TokenType, Localizable as L, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::typed_identifier_list_parser::parse_typed_identifier_list;

pub fn parse_parameters<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<Parameters>, NilangError> {
    let start = tokens.assume(TokenType::OpeningParenthesis)?;

    let fields = parse_typed_identifier_list(tokens)?.object;

    let L { object, location } = tokens.assume_next()?;
    if object == TokenType::ClosingParenthesis {
        Ok(L {
            location: Location::between(&start, &location),
            object: fields,
        })
    } else {
        Err(NilangError {
            location,
            error: ParserErrors::ExpectedTokens(Vec::from([
                if fields.is_empty() {
                    TokenType::Identifier("".into())
                } else {
                    TokenType::Comma
                },
                TokenType::ClosingParenthesis,
            ]))
            .into(),
        })
    }
}

#[cfg(test)]
mod test {
    use nilang_types::{nodes::Type, tokens::TokenType, Localizable as L};

    use crate::parsers::parameters_parser::parse_parameters;

    #[test]
    fn test_parameter_list() {
        assert_eq!(
            parse_parameters(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Identifier("test1".into()),)),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Comma,)),
                    Ok(L::irrelevant(TokenType::Identifier("test2".into()),)),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            [
                (L::irrelevant("test1".into()), L::irrelevant(Type::Int)),
                (L::irrelevant("test2".into()), L::irrelevant(Type::Int)),
            ]
            .into()
        );
    }
}
