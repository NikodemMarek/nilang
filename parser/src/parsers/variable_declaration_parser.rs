use errors::NilangError;
use nilang_types::{
    nodes::StatementNode,
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type_annotation};

pub fn parse_variable_declaration<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<StatementNode>, NilangError> {
    let start = tokens.assume_keyword(Keyword::Variable)?;

    let name = tokens.assume_identifier()?;

    let r#type = parse_type_annotation(tokens)?;

    tokens.assume(TokenType::Equals)?;

    let value = parse_expression(tokens)?;

    let end = tokens.assume(TokenType::Semicolon)?;

    Ok(Localizable::new(
        Location::between(&start, &end),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value: Box::new(value),
        },
    ))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionCall, Operator, StatementNode, Type},
        tokens::{Keyword, TokenType},
        Localizable,
    };

    use crate::parsers::variable_declaration_parser::parse_variable_declaration;

    #[test]
    fn test_parse_variable_declaration() {
        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Variable
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Equals,)),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: Localizable::irrelevant("test".into()),
                r#type: Localizable::irrelevant(Type::Int),
                value: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.)))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Variable
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Equals,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test2".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: Localizable::irrelevant("test".into()),
                r#type: Localizable::irrelevant(Type::Int),
                value: Box::new(Localizable::irrelevant(ExpressionNode::VariableReference(
                    Localizable::irrelevant("test2".into())
                )))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Variable
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Equals,)),
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: Localizable::irrelevant("test".into()),
                r#type: Localizable::irrelevant(Type::Int),
                value: Box::new(Localizable::irrelevant(ExpressionNode::Operation {
                    operator: Localizable::irrelevant(Operator::Add),
                    a: Box::new(Localizable::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                }))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Variable
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Equals,)),
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test2".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: Localizable::irrelevant("test".into()),
                r#type: Localizable::irrelevant(Type::Int),
                value: Box::new(Localizable::irrelevant(ExpressionNode::Operation {
                    operator: Localizable::irrelevant(Operator::Add),
                    a: Box::new(Localizable::irrelevant(ExpressionNode::VariableReference(
                        Localizable::irrelevant("test2".into())
                    ))),
                    b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                }))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Variable
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ))),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Equals,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("abc".into()),)),
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: Localizable::irrelevant("test".into()),
                r#type: Localizable::irrelevant(Type::Int),
                value: Box::new(Localizable::irrelevant(ExpressionNode::FunctionCall(
                    Localizable::irrelevant(FunctionCall {
                        name: Localizable::irrelevant("abc".into()),
                        arguments: Localizable::irrelevant(
                            [Localizable::irrelevant(ExpressionNode::Operation {
                                operator: Localizable::irrelevant(Operator::Add),
                                a: Box::new(Localizable::irrelevant(ExpressionNode::Number(6.))),
                                b: Box::new(Localizable::irrelevant(ExpressionNode::Number(9.))),
                            })]
                            .into()
                        )
                    })
                )))
            }
        );
    }
}
