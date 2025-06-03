use errors::NilangError;
use nilang_types::{
    nodes::{ExpressionNode, FunctionCall, StatementNode},
    tokens::TokenType,
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    arguments_parser::parse_arguments, operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_function_call_statement<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: L<Box<str>>,
) -> Result<L<StatementNode>, NilangError> {
    let function_call = parse_function_call_only(tokens, name)?;
    Ok(L::new(
        function_call.location,
        StatementNode::FunctionCall(function_call.object),
    ))
}

pub fn parse_function_call_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: L<Box<str>>,
) -> Result<L<ExpressionNode>, NilangError> {
    let function_call = parse_function_call_only(tokens, name)?;
    let function_call_expression = L::new(
        function_call.location,
        ExpressionNode::FunctionCall(function_call.object),
    );
    let function_call_field_access =
        expand_function_call_if_dot_follows(tokens, function_call_expression)?;
    parse_operation_if_operator_follows(tokens, function_call_field_access)
}

fn parse_function_call_only<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: L<Box<str>>,
) -> Result<L<FunctionCall>, NilangError> {
    let arguments = parse_arguments(tokens)?;

    let location = Location::between(&name.location, &arguments.location.clone());
    Ok(L::new(location, FunctionCall { name, arguments }))
}

fn expand_function_call_if_dot_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    function_call: L<ExpressionNode>, // only FunctionCall is allowed here
) -> Result<L<ExpressionNode>, NilangError> {
    if let TokenType::Dot = tokens.peek_valid()?.object {
        tokens.assume(TokenType::Dot)?;

        let field = tokens.assume_identifier()?;

        Ok(L::new(
            Location::between(&function_call.location, &field.location),
            ExpressionNode::FieldAccess {
                structure: Box::new(function_call),
                field,
            },
        ))
    } else {
        Ok(function_call)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionCall},
        tokens::TokenType,
        Localizable as L,
    };

    use crate::parsers::function_call_parser::parse_function_call_expression;

    #[test]
    fn test_parse_function_call() {
        assert_eq!(
            parse_function_call_expression(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Semicolon))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FunctionCall(FunctionCall {
                name: L::irrelevant("x".into()),
                arguments: L::irrelevant([].into())
            })
        );

        assert_eq!(
            parse_function_call_expression(
                &mut [
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Dot,)),
                    Ok(L::irrelevant(TokenType::Identifier("test".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                L::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(L::irrelevant(ExpressionNode::FunctionCall(FunctionCall {
                    name: L::irrelevant("x".into()),
                    arguments: L::irrelevant([].into())
                }))),
                field: L::irrelevant("test".into())
            }
        );
    }
}
