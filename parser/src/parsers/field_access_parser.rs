use errors::NilangError;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_field_access<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Box<str>,
) -> Result<ExpressionNode, NilangError> {
    let mut field_access = ExpressionNode::VariableReference(name);

    while let TokenType::Dot = tokens.peek_valid()?.token {
        tokens.assume(TokenType::Dot)?;

        let (_, _, subfield) = tokens.assume_identifier()?;

        field_access = ExpressionNode::FieldAccess {
            structure: Box::new(field_access),
            field: subfield,
        }
    }

    Ok(field_access)
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::ExpressionNode,
        tokens::{Token, TokenType},
    };

    use crate::parsers::field_access_parser::parse_field_access;

    #[test]
    fn test_parse_field_access() {
        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                ]
                .into_iter()
                .peekable(),
                "x".into()
            )
            .unwrap(),
            ExpressionNode::FieldAccess {
                structure: Box::new(ExpressionNode::VariableReference("x".into())),
                field: "test".into()
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test1".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 7),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 12),
                        end: (0, 12),
                    }),
                ]
                .into_iter()
                .peekable(),
                "x".into()
            )
            .unwrap(),
            ExpressionNode::FieldAccess {
                structure: Box::new(ExpressionNode::FieldAccess {
                    structure: Box::new(ExpressionNode::VariableReference("x".into())),
                    field: "test1".into()
                }),
                field: "test2".into()
            }
        );

        assert_eq!(
            parse_field_access(
                &mut [
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test1".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 7),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 12),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test3".into()),
                        start: (0, 13),
                        end: (0, 17),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 18),
                        end: (0, 18),
                    }),
                ]
                .into_iter()
                .peekable(),
                "x".into()
            )
            .unwrap(),
            ExpressionNode::FieldAccess {
                structure: Box::new(ExpressionNode::FieldAccess {
                    structure: Box::new(ExpressionNode::FieldAccess {
                        structure: Box::new(ExpressionNode::VariableReference("x".into())),
                        field: "test1".into()
                    }),
                    field: "test2".into()
                }),
                field: "test3".into()
            }
        );
    }
}
