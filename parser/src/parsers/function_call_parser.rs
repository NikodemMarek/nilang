use errors::NilangError;
use nilang_types::{
    nodes::{ExpressionNode, FunctionCall, StatementNode},
    tokens::TokenType,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    argument_list_parser::parse_argument_list,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_function_call_statement<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    let function_call = parse_function_call_only(tokens)?;
    tokens.assume(TokenType::Semicolon)?;
    Ok(StatementNode::FunctionCall(function_call))
}

pub fn parse_function_call_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, NilangError> {
    let function_call = parse_function_call_only(tokens)?;
    let function_call_field_access =
        expand_function_call_if_dot_follows(tokens, ExpressionNode::FunctionCall(function_call))?;
    parse_operation_if_operator_follows(tokens, function_call_field_access)
}

fn parse_function_call_only<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<FunctionCall, NilangError> {
    let (_, _, name) = tokens.assume_identifier()?;
    let arguments = parse_argument_list(tokens)?;

    Ok(FunctionCall { name, arguments })
}

fn expand_function_call_if_dot_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    function_call: ExpressionNode, // only FunctionCall is allowed here
) -> Result<ExpressionNode, NilangError> {
    if let TokenType::Dot = tokens.peek_valid()?.token {
        tokens.assume(TokenType::Dot)?;

        let (_, _, field) = tokens.assume_identifier()?;

        Ok(ExpressionNode::FieldAccess {
            structure: Box::new(function_call),
            field,
        })
    } else {
        Ok(function_call)
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionCall},
        tokens::{Token, TokenType},
    };

    use crate::{
        multi_peekable::MultiPeekable,
        parsers::function_call_parser::parse_function_call_expression,
    };

    #[test]
    fn test_parse_function_call() {
        assert_eq!(
            parse_function_call_expression(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 3),
                        end: (0, 3),
                    })
                ]
                .into_iter()
            ))
            .unwrap(),
            ExpressionNode::FunctionCall(FunctionCall {
                name: "x".into(),
                arguments: [].into()
            })
        );

        assert_eq!(
            parse_function_call_expression(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Dot,
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 4),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 8),
                        end: (0, 8),
                    })
                ]
                .into_iter()
            ))
            .unwrap(),
            ExpressionNode::FieldAccess {
                structure: Box::new(ExpressionNode::FunctionCall(FunctionCall {
                    name: "x".into(),
                    arguments: [].into()
                })),
                field: "test".into()
            }
        );
    }
}
