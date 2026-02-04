use errors::NilangError;
use nilang_types::{
    nodes::expressions::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    field_access_parser::parse_field_access, function_call_parser::parse_function_call_expression,
    object_parser::parse_object, operation::lookup_operation_recursive,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, NilangError> {
    let expression = match tokens.peek_nth_valid(1)? {
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => parse_function_call_expression(tokens)?,
        Token {
            token: TokenType::Operator(_),
            ..
        } => {
            let (_, _, name) = tokens.assume_identifier()?;
            lookup_operation_recursive(tokens, ExpressionNode::VariableReference(name))?
        }
        Token {
            token: TokenType::OpeningBrace,
            ..
        } => parse_object(tokens)?,
        Token {
            token: TokenType::Dot,
            ..
        } => parse_field_access(tokens)?,
        Token { .. } => {
            let (_, _, name) = tokens.assume_identifier()?;
            ExpressionNode::VariableReference(name)
        }
    };

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::{multi_peekable::MultiPeekable, parsers::identifier_parser::parse_identifier};
    use nilang_types::{
        nodes::expressions::ExpressionNode,
        tokens::{Token, TokenType},
    };

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier(&mut MultiPeekable::new(
                [
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
            ))
            .unwrap(),
            ExpressionNode::VariableReference("x".into())
        );
    }
}
