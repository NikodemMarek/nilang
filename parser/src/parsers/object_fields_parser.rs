use std::collections::HashMap;

use errors::NilangError;
use nilang_types::{
    nodes::{ExpressionNode, ObjectFields, Str},
    tokens::TokenType,
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_object_fields<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<ObjectFields>, NilangError> {
    let start = tokens.peek_valid()?.location;

    let mut list = HashMap::new();
    while let L {
        object: TokenType::Identifier(_),
        ..
    } = tokens.peek_valid()?
    {
        let (name, expression) = parse_object_field(tokens)?;
        list.insert(name, expression);

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

    Ok(L::new(Location::between(&start, &end), list))
}

pub fn parse_object_field<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<(L<Str>, L<ExpressionNode>), NilangError> {
    let name = tokens.assume_identifier()?;
    tokens.assume(TokenType::Colon)?;
    let expression = parse_expression(tokens)?;
    Ok((name, expression))
}
