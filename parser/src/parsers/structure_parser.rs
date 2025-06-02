use std::collections::HashMap;

use errors::{NilangError, ParserErrors};
use nilang_types::{
    nodes::StructureDeclaration,
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_type_annotation;

pub fn parse_structure<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StructureDeclaration, NilangError> {
    tokens.assume_keyword(Keyword::Structure)?;

    let name = tokens.assume_identifier()?;

    let start = tokens.assume(TokenType::OpeningBrace)?;

    let mut fields = HashMap::new();

    loop {
        let name = tokens.assume_identifier()?;
        let r#type = parse_type_annotation(tokens)?;

        fields.insert(name, r#type);

        match tokens.peek_valid()? {
            Localizable {
                object: TokenType::Comma,
                ..
            } => {
                let _ = tokens.assume(TokenType::Comma);

                if let Localizable {
                    object: TokenType::ClosingBrace,
                    ..
                } = tokens.peek_valid()?
                {
                    break;
                }
            }
            Localizable {
                object: TokenType::ClosingBrace,
                ..
            } => break,
            Localizable { location, .. } => {
                return Err(NilangError {
                    location: *location,
                    error: ParserErrors::ExpectedTokens(vec![
                        TokenType::Comma,
                        TokenType::ClosingBrace,
                    ])
                    .into(),
                });
            }
        }
    }

    let end = tokens.assume(TokenType::ClosingBrace)?;

    let fields = Localizable::new(Location::between(&start, &end), fields);
    Ok(StructureDeclaration { name, fields })
}

#[cfg(test)]
mod test {
    use nilang_types::{
        nodes::{StructureDeclaration, Type},
        tokens::{Keyword, TokenType},
        Localizable,
    };

    use crate::parsers::structure_parser::parse_structure;

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_structure(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Structure,
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "Test".into(),
                    ))),
                    Ok(Localizable::irrelevant(TokenType::OpeningBrace)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test_field".into(),
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Comma)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test_field2".into(),
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingBrace)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            StructureDeclaration {
                name: Localizable::irrelevant("Test".into()),
                fields: Localizable::irrelevant(
                    [
                        (
                            Localizable::irrelevant("test_field".into()),
                            Localizable::irrelevant(Type::Int)
                        ),
                        (
                            Localizable::irrelevant("test_field2".into()),
                            Localizable::irrelevant(Type::Int)
                        ),
                    ]
                    .into()
                ),
            }
        );

        assert_eq!(
            parse_structure(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Structure,
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "Test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::OpeningBrace)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test_field".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()))),
                    Ok(Localizable::irrelevant(TokenType::Comma)),
                    Ok(Localizable::irrelevant(TokenType::ClosingBrace)),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            StructureDeclaration {
                name: Localizable::irrelevant("Test".into()),
                fields: Localizable::irrelevant(
                    [(
                        Localizable::irrelevant("test_field".into()),
                        Localizable::irrelevant(Type::Int)
                    )]
                    .into()
                ),
            },
        );
    }
}
