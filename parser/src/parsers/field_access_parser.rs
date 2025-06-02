use errors::NilangError;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable, Location};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_field_access<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Localizable<Box<str>>,
) -> Result<Localizable<ExpressionNode>, NilangError> {
    let mut field_access = Localizable::new(
        name.location,
        ExpressionNode::VariableReference(name.object),
    );

    while let TokenType::Dot = tokens.peek_valid()?.object {
        tokens.assume(TokenType::Dot)?;

        let subfield = tokens.assume_identifier()?;

        field_access = Localizable::new(
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
    use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable};

    use crate::parsers::field_access_parser::parse_field_access;

    #[test]
    fn test_parse_field_access() {
        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                Localizable::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(Localizable::irrelevant(ExpressionNode::VariableReference(
                    "x".into()
                ))),
                field: Localizable::irrelevant("test".into())
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test1".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test2".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                Localizable::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(Localizable::irrelevant(ExpressionNode::FieldAccess {
                    structure: Box::new(Localizable::irrelevant(
                        ExpressionNode::VariableReference("x".into())
                    )),
                    field: Localizable::irrelevant("test1".into())
                })),
                field: Localizable::irrelevant("test2".into())
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test1".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test2".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test3".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
                Localizable::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(Localizable::irrelevant(ExpressionNode::FieldAccess {
                    structure: Box::new(Localizable::irrelevant(ExpressionNode::FieldAccess {
                        structure: Box::new(Localizable::irrelevant(
                            ExpressionNode::VariableReference("x".into())
                        )),
                        field: Localizable::irrelevant("test1".into())
                    })),
                    field: Localizable::irrelevant("test2".into())
                })),
                field: Localizable::irrelevant("test3".into())
            }
        );
    }
}
