use errors::NilangError;
use nilang_types::{
    nodes::{ExpressionNode, Type},
    tokens::TokenType,
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::object_fields_parser::parse_object_fields;

pub fn parse_object<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: L<Box<str>>,
) -> Result<L<ExpressionNode>, NilangError> {
    let start = tokens.assume(TokenType::OpeningBrace)?;

    let fields = parse_object_fields(tokens)?;

    let end = tokens.assume(TokenType::ClosingBrace)?;

    let r#type = L::new(name.location, Type::Object((*name).clone()));
    Ok(L::new(
        Location::between(&start, &end),
        ExpressionNode::Object { r#type, fields },
    ))
}
