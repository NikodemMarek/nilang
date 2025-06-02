use std::collections::HashMap;

use errors::{NilangError, ParserErrors};
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type};

pub fn parse_object<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Localizable<Box<str>>,
) -> Result<Localizable<ExpressionNode>, NilangError> {
    let start = tokens.assume(TokenType::OpeningBrace)?;

    let mut fields = HashMap::new();

    loop {
        match tokens.assume_next()? {
            Localizable {
                object: TokenType::Identifier(field_name),
                location,
            } => {
                tokens.assume(TokenType::Colon)?;

                if fields
                    .insert(
                        Localizable::new(location, field_name.clone()),
                        parse_expression(tokens)?,
                    )
                    .is_some()
                {
                    return Err(NilangError {
                        location,
                        error: ParserErrors::DuplicateField(field_name).into(),
                    });
                };

                match tokens.assume_next()? {
                    Localizable {
                        object: TokenType::Comma,
                        ..
                    } => {}
                    Localizable {
                        object: TokenType::ClosingBrace,
                        location: end,
                    } => {
                        return Ok(Localizable::new(
                            Location::between(&name.location, &end),
                            ExpressionNode::Object {
                                r#type: Localizable::new(name.location, parse_type(&field_name)),
                                fields: Localizable::new(Location::between(&start, &end), fields),
                            },
                        ));
                    }
                    Localizable { location, .. } => Err(NilangError {
                        location,
                        error: ParserErrors::ExpectedTokens(Vec::from([
                            TokenType::Comma,
                            TokenType::ClosingBrace,
                        ]))
                        .into(),
                    })?,
                }
            }
            Localizable {
                object: TokenType::ClosingBrace,
                location: end,
            } => {
                return Ok(Localizable::new(
                    Location::between(&name.location, &end),
                    ExpressionNode::Object {
                        r#type: Localizable::new(name.location, parse_type(&name)),
                        fields: Localizable::new(Location::between(&start, &end), fields),
                    },
                ));
            }
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingBrace,
                ]))
                .into(),
            })?,
        }
    }
}
