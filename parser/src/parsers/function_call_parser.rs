use errors::ParserErrors;
use nilang_types::{nodes::ExpressionNode, tokens::TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    argument_list_parser::parse_argument_list,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_function_call<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Box<str>,
) -> Result<ExpressionNode, ParserErrors> {
    let function_call = parse_function_call_only(tokens, name)?;
    let function_call_field_access = expand_function_call_if_dot_follows(tokens, function_call)?;
    parse_operation_if_operator_follows(tokens, function_call_field_access)
}

fn parse_function_call_only<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Box<str>,
) -> Result<ExpressionNode, ParserErrors> {
    let arguments = parse_argument_list(tokens)?;

    Ok(ExpressionNode::FunctionCall { name, arguments })
}

fn expand_function_call_if_dot_follows<I: PeekableAssumingIterator>(
    tokens: &mut I,
    function_call: ExpressionNode, // only FunctionCall is allowed here
) -> Result<ExpressionNode, ParserErrors> {
    if let TokenType::Dot = tokens.peek_valid()?.token {
        tokens.assume_dot()?;
        let field = tokens.assume_identifier()?.2;
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
        nodes::ExpressionNode,
        tokens::{Token, TokenType},
    };

    use crate::parsers::function_call_parser::parse_function_call;

    #[test]
    fn test_parse_function_call() {
        assert_eq!(
            parse_function_call(
                &mut [
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
                .peekable(),
                "x".into()
            )
            .unwrap(),
            ExpressionNode::FunctionCall {
                name: "x".into(),
                arguments: [].into()
            }
        );

        assert_eq!(
            parse_function_call(
                &mut [
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
                .peekable(),
                "x".into()
            )
            .unwrap(),
            ExpressionNode::FieldAccess {
                structure: Box::new(ExpressionNode::FunctionCall {
                    name: "x".into(),
                    arguments: [].into()
                }),
                field: "test".into()
            }
        );
    }
}
