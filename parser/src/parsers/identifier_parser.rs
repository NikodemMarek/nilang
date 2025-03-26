use errors::ParserErrors;
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    field_access_parser::parse_field_access, function_call_parser::parse_function_call_expression,
    object_parser::parse_object, operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, ParserErrors> {
    let (_, _, name) = tokens.assume_identifier()?;

    let peek_valid = if let Ok(token) = tokens.peek_valid() {
        token
    } else {
        return Ok(ExpressionNode::VariableReference(name));
    };

    let expression = match peek_valid {
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => parse_function_call_expression(tokens, name)?,
        Token {
            token: TokenType::Operator(_),
            ..
        } => parse_operation_if_operator_follows(tokens, ExpressionNode::VariableReference(name))?,
        Token {
            token: TokenType::OpeningBrace,
            ..
        } => parse_object(tokens, name)?,
        Token {
            token: TokenType::Dot,
            ..
        } => parse_field_access(tokens, name)?,
        Token { .. } => ExpressionNode::VariableReference(name),
    };

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::parsers::identifier_parser::parse_identifier;
    use nilang_types::{
        nodes::ExpressionNode,
        tokens::{Token, TokenType},
    };

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier(
                &mut [
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 1),
                        end: (0, 1),
                    })
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            ExpressionNode::VariableReference("x".into())
        );
    }
}
