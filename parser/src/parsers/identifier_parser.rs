use errors::NilangError;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    field_access_parser::parse_field_access, function_call_parser::parse_function_call_expression,
    object_parser::parse_object, operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<ExpressionNode>, NilangError> {
    let name = tokens.assume_identifier()?;

    let expression = match tokens.peek_valid()? {
        Localizable {
            object: TokenType::OpeningParenthesis,
            ..
        } => parse_function_call_expression(tokens, name)?,
        Localizable {
            object: TokenType::Operator(_),
            ..
        } => parse_operation_if_operator_follows(
            tokens,
            Localizable::new(
                name.location,
                ExpressionNode::VariableReference(name.object),
            ),
        )?,
        Localizable {
            object: TokenType::OpeningBrace,
            ..
        } => parse_object(tokens, name)?,
        Localizable {
            object: TokenType::Dot,
            ..
        } => parse_field_access(tokens, name)?,
        Localizable { .. } => Localizable::new(
            name.location,
            ExpressionNode::VariableReference(name.object),
        ),
    };

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::parsers::identifier_parser::parse_identifier;
    use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable};

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            ExpressionNode::VariableReference("x".into())
        );
    }
}
