use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::Parameter, tokens::TokenType, Localizable, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_type_annotation;

pub fn parse_parameter_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<Box<[Parameter]>>, NilangError> {
    let start = tokens.assume(TokenType::OpeningParenthesis)?;

    let mut parameters = Vec::new();

    loop {
        match tokens.assume_next()? {
            Localizable {
                object: TokenType::Identifier(value),
                location,
            } => {
                let parameter_name = Localizable::new(location, value);
                let parameter_type = parse_type_annotation(tokens)?;
                parameters.push((parameter_name, parameter_type));

                match tokens.assume_next()? {
                    Localizable {
                        object: TokenType::ClosingParenthesis,
                        location: end,
                    } => {
                        return Ok(Localizable {
                            location: Location::between(&start, &end),
                            object: parameters.into(),
                        });
                    }
                    Localizable {
                        object: TokenType::Comma,
                        ..
                    } => {}
                    Localizable { location, .. } => Err(NilangError {
                        location,
                        error: ParserErrors::ExpectedTokens(Vec::from([
                            TokenType::Comma,
                            TokenType::ClosingParenthesis,
                        ]))
                        .into(),
                    })?,
                }
            }
            Localizable {
                object: TokenType::ClosingParenthesis,
                location: end,
            } => {
                return Ok(Localizable {
                    location: Location::between(&start, &end),
                    object: parameters.into(),
                });
            }
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingParenthesis,
                ]))
                .into(),
            })?,
        }
    }
}

#[cfg(test)]
mod test {
    use nilang_types::{nodes::Type, tokens::TokenType, Localizable};

    use crate::parsers::parameter_list_parser::parse_parameter_list;

    #[test]
    fn test_parameter_list() {
        assert_eq!(
            parse_parameter_list(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test1".into()
                    ),)),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Comma,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test2".into()
                    ),)),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            [
                (
                    Localizable::irrelevant("test1".into()),
                    Localizable::irrelevant(Type::Int)
                ),
                (
                    Localizable::irrelevant("test2".into()),
                    Localizable::irrelevant(Type::Int)
                ),
            ]
            .into()
        );
    }
}
