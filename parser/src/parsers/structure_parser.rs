use std::collections::HashMap;

use errors::{NilangError, ParserErrors};
use nilang_types::{
    nodes::{StructureDeclaration, StructureFields},
    tokens::{Keyword, TokenType},
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::typed_identifier_list_parser::parse_typed_identifier_list;

pub fn parse_structure<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<StructureDeclaration>, NilangError> {
    tokens.assume_keyword(Keyword::Structure)?;

    let name = tokens.assume_identifier()?;

    let fields = parse_structure_fields(tokens)?;

    Ok(L::new(
        Location::between(&name.location, &fields.location),
        StructureDeclaration { name, fields },
    ))
}

pub fn parse_structure_fields<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<StructureFields>, NilangError> {
    let start = tokens.assume(TokenType::OpeningBrace)?;

    let fields = parse_typed_identifier_list(tokens)?
        .object
        .iter()
        .map(|(name, r#type)| (name.to_owned(), r#type.to_owned()))
        .collect::<HashMap<_, _>>();

    let L { object, location } = tokens.assume_next()?;
    if object == TokenType::ClosingBrace {
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
                TokenType::ClosingBrace,
            ]))
            .into(),
        })
    }
}

#[cfg(test)]
mod test {
    use nilang_types::{
        nodes::{StructureDeclaration, Type},
        tokens::{Keyword, TokenType},
        Localizable as L,
    };

    use crate::parsers::structure_parser::parse_structure;

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_structure(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Structure,))),
                    Ok(L::irrelevant(TokenType::Identifier("Test".into(),))),
                    Ok(L::irrelevant(TokenType::OpeningBrace)),
                    Ok(L::irrelevant(TokenType::Identifier("test_field".into(),))),
                    Ok(L::irrelevant(TokenType::Colon)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Comma)),
                    Ok(L::irrelevant(TokenType::Identifier("test_field2".into(),))),
                    Ok(L::irrelevant(TokenType::Colon)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingBrace)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            StructureDeclaration {
                name: L::irrelevant("Test".into()),
                fields: L::irrelevant(
                    [
                        (L::irrelevant("test_field".into()), L::irrelevant(Type::Int)),
                        (
                            L::irrelevant("test_field2".into()),
                            L::irrelevant(Type::Int)
                        ),
                    ]
                    .into()
                ),
            }
        );

        assert_eq!(
            parse_structure(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Structure,))),
                    Ok(L::irrelevant(TokenType::Identifier("Test".into()))),
                    Ok(L::irrelevant(TokenType::OpeningBrace)),
                    Ok(L::irrelevant(TokenType::Identifier("test_field".into()))),
                    Ok(L::irrelevant(TokenType::Colon)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()))),
                    Ok(L::irrelevant(TokenType::Comma)),
                    Ok(L::irrelevant(TokenType::ClosingBrace)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            StructureDeclaration {
                name: L::irrelevant("Test".into()),
                fields: L::irrelevant(
                    [(L::irrelevant("test_field".into()), L::irrelevant(Type::Int))].into()
                ),
            },
        );
    }
}
