use errors::NilangError;
use nilang_types::{nodes::TypedIdentifier, tokens::TokenType, Localizable as L, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_typed_identifier;

pub fn parse_typed_identifier_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<Box<[TypedIdentifier]>>, NilangError> {
    let start = tokens.peek_valid()?.location;

    let mut list = Vec::new();
    while let L {
        object: TokenType::Identifier(_),
        ..
    } = tokens.peek_valid()?
    {
        list.push(parse_typed_identifier(tokens)?);

        if let L {
            object: TokenType::Comma,
            ..
        } = tokens.peek_valid()?
        {
            tokens.assume_next()?;
        } else {
            break;
        }
    }

    let end = tokens.peek_valid()?.location;

    Ok(L::new(
        Location::between(&start, &end),
        list.into_boxed_slice(),
    ))
}
