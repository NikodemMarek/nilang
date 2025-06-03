use errors::NilangError;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable as L};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    field_access_parser::parse_field_access, function_call_parser::parse_function_call_expression,
    object_parser::parse_object, operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<ExpressionNode>, NilangError> {
    let name = tokens.assume_identifier()?;

    let expression = match tokens.peek_valid()? {
        L {
            object: TokenType::OpeningParenthesis,
            ..
        } => parse_function_call_expression(tokens, name)?,
        L {
            object: TokenType::Operator(_),
            ..
        } => parse_operation_if_operator_follows(
            tokens,
            L::new(
                name.location,
                ExpressionNode::VariableReference(name.object),
            ),
        )?,
        L {
            object: TokenType::OpeningBrace,
            ..
        } => parse_object(tokens, name)?,
        L {
            object: TokenType::Dot,
            ..
        } => parse_field_access(tokens, name)?,
        L { .. } => L::new(
            name.location,
            ExpressionNode::VariableReference(name.object),
        ),
    };

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::parsers::identifier_parser::parse_identifier;
    use nilang_types::{nodes::ExpressionNode, tokens::TokenType, Localizable as L};

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier(
                &mut [
                    Ok(L::irrelevant(TokenType::Identifier("x".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
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
