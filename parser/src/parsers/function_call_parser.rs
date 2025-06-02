use errors::NilangError;
use nilang_types::{
    nodes::{ExpressionNode, FunctionCall, StatementNode},
    tokens::TokenType,
    Localizable, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    argument_list_parser::parse_argument_list,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_function_call_statement<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Localizable<Box<str>>,
) -> Result<Localizable<StatementNode>, NilangError> {
    let function_call = parse_function_call_only(tokens, name)?;
    Ok(Localizable::new(
        function_call.location,
        StatementNode::FunctionCall(function_call.object),
    ))
}

pub fn parse_function_call_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Localizable<Box<str>>,
) -> Result<Localizable<ExpressionNode>, NilangError> {
    let function_call = parse_function_call_only(tokens, name)?;
    let function_call_expression = Localizable::new(
        function_call.location,
        ExpressionNode::FunctionCall(function_call.object),
    );
    let function_call_field_access =
        expand_function_call_if_dot_follows(tokens, function_call_expression)?;
    parse_operation_if_operator_follows(tokens, function_call_field_access)
}

fn parse_function_call_only<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Localizable<Box<str>>,
) -> Result<Localizable<FunctionCall>, NilangError> {
    let arguments = parse_argument_list(tokens)?;

    let location = Location::between(&name.location, &arguments.location.clone());
    Ok(Localizable::new(location, FunctionCall { name, arguments }))
}

fn expand_function_call_if_dot_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    function_call: Localizable<ExpressionNode>, // only FunctionCall is allowed here
) -> Result<Localizable<ExpressionNode>, NilangError> {
    if let TokenType::Dot = tokens.peek_valid()?.object {
        tokens.assume(TokenType::Dot)?;

        let field = tokens.assume_identifier()?;

        Ok(Localizable::new(
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
        Localizable,
    };

    use crate::parsers::function_call_parser::parse_function_call_expression;

    #[test]
    fn test_parse_function_call() {
        assert_eq!(
            parse_function_call_expression(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon))
                ]
                .into_iter()
                .peekable(),
                Localizable::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FunctionCall(FunctionCall {
                name: Localizable::irrelevant("x".into()),
                arguments: Localizable::irrelevant([].into())
            })
        );

        assert_eq!(
            parse_function_call_expression(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Dot,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "test".into()
                    ),)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,))
                ]
                .into_iter()
                .peekable(),
                Localizable::irrelevant("x".into())
            )
            .unwrap()
            .object,
            ExpressionNode::FieldAccess {
                structure: Box::new(Localizable::irrelevant(ExpressionNode::FunctionCall(
                    FunctionCall {
                        name: Localizable::irrelevant("x".into()),
                        arguments: Localizable::irrelevant([].into())
                    }
                ))),
                field: Localizable::irrelevant("test".into())
            }
        );
    }
}
