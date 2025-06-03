use errors::NilangError;
use nilang_types::{
    nodes::StatementNode,
    tokens::{Keyword, TokenType},
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type_annotation};

pub fn parse_variable_declaration<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<StatementNode>, NilangError> {
    let start = tokens.assume_keyword(Keyword::Variable)?;

    let name = tokens.assume_identifier()?;

    let r#type = parse_type_annotation(tokens)?;

    tokens.assume(TokenType::Equals)?;

    let value = parse_expression(tokens)?;

    let end = tokens.assume(TokenType::Semicolon)?;

    Ok(L::new(
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
        Localizable as L,
    };

    use crate::parsers::variable_declaration_parser::parse_variable_declaration;

    #[test]
    fn test_parse_variable_declaration() {
        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Variable))),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Equals,)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: L::irrelevant("test".into()),
                r#type: L::irrelevant(Type::Int),
                value: Box::new(L::irrelevant(ExpressionNode::Number(9.)))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Variable))),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Equals,)),
                    Ok(L::irrelevant(TokenType::Identifier("test2".into()))),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: L::irrelevant("test".into()),
                r#type: L::irrelevant(Type::Int),
                value: Box::new(L::irrelevant(ExpressionNode::VariableReference(
                    "test2".into()
                )))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Variable))),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Equals,)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: L::irrelevant("test".into()),
                r#type: L::irrelevant(Type::Int),
                value: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                }))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Variable))),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Equals,)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Identifier("test2".into()))),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: L::irrelevant("test".into()),
                r#type: L::irrelevant(Type::Int),
                value: Box::new(L::irrelevant(ExpressionNode::Operation {
                    operator: L::irrelevant(Operator::Add),
                    a: Box::new(L::irrelevant(ExpressionNode::VariableReference(
                        "test2".into()
                    ))),
                    b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                }))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Variable))),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()))),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::Equals,)),
                    Ok(L::irrelevant(TokenType::Identifier("abc".into()),)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Operator(Operator::Add),)),
                    Ok(L::irrelevant(TokenType::Literal("9".into()),)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            StatementNode::VariableDeclaration {
                name: L::irrelevant("test".into()),
                r#type: L::irrelevant(Type::Int),
                value: Box::new(L::irrelevant(ExpressionNode::FunctionCall(FunctionCall {
                    name: L::irrelevant("abc".into()),
                    arguments: L::irrelevant(
                        [L::irrelevant(ExpressionNode::Operation {
                            operator: L::irrelevant(Operator::Add),
                            a: Box::new(L::irrelevant(ExpressionNode::Number(6.))),
                            b: Box::new(L::irrelevant(ExpressionNode::Number(9.))),
                        })]
                        .into()
                    )
                })))
            }
        );
    }
}
