use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    argument_list_parser::parse_argument_list,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    let (_, _, value) = tokens.assume_identifier()?;

    match tokens.peek_valid()? {
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => {
            let function_arguments = parse_argument_list(tokens);
            parse_operation_if_operator_follows(
                tokens,
                Node::FunctionCall {
                    name: value.to_string(),
                    arguments: function_arguments?,
                },
            )
        }
        Token {
            token: TokenType::Operator(_),
            ..
        } => {
            parse_operation_if_operator_follows(tokens, Node::VariableReference(value.to_string()))
        }
        Token { .. } => Ok(Node::VariableReference(value.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::identifier_parser::parse_identifier;
    use nilang_types::{
        nodes::Node,
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
            Node::VariableReference("x".to_string())
        );

        assert_eq!(
            parse_identifier(
                &mut [
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
                .peekable()
            )
            .unwrap(),
            Node::FunctionCall {
                name: "x".to_string(),
                arguments: Vec::new()
            }
        );
    }
}
