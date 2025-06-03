use errors::NilangError;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable as L, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_field_access<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: L<Box<str>>,
) -> Result<L<ExpressionNode>, NilangError> {
    let mut field_access = L::new(
        name.location,
        ExpressionNode::VariableReference(name.object),
    );

    while let TokenType::Dot = tokens.peek_valid()?.object {
        tokens.assume(TokenType::Dot)?;

        let subfield = tokens.assume_identifier()?;

        field_access = L::new(
            Location::between(&field_access.location, &subfield.location),
            ExpressionNode::FieldAccess {
                structure: Box::new(field_access),
                field: subfield,
            },
        )
    }

    Ok(field_access)
}

#[cfg(test)]
mod tests {
    use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable as L};

    use crate::parsers::field_access_parser::parse_field_access;

    #[test]
    fn test_parse_field_access() {
        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                L::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(L::irrelevant(ExpressionNode::VariableReference("x".into()))),
                field: L::irrelevant("test".into())
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test1".into()))),
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test2".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                L::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(L::irrelevant(ExpressionNode::FieldAccess {
                    structure: Box::new(L::irrelevant(ExpressionNode::VariableReference(
                        "x".into()
                    ))),
                    field: L::irrelevant("test1".into())
                })),
                field: L::irrelevant("test2".into())
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test1".into()))),
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test2".into()))),
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test3".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                L::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(L::irrelevant(ExpressionNode::FieldAccess {
                    structure: Box::new(L::irrelevant(ExpressionNode::FieldAccess {
                        structure: Box::new(L::irrelevant(ExpressionNode::VariableReference(
                            "x".into()
                        ))),
                        field: L::irrelevant("test1".into())
                    })),
                    field: L::irrelevant("test2".into())
                })),
                field: L::irrelevant("test3".into())
            }
        );
    }
}
