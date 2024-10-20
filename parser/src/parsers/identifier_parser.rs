use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{
    function_arguments_parser::parse_function_arguments,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    let value = if let Some(Ok(Token {
        token: TokenType::Identifier(value),
        ..
    })) = tokens.next()
    {
        value
    } else {
        unreachable!()
    };

    Ok(match tokens.peek() {
        Some(Ok(Token {
            token: TokenType::OpeningParenthesis,
            ..
        })) => {
            let function_arguments = parse_function_arguments(tokens);
            parse_operation_if_operator_follows(
                tokens,
                Node::FunctionCall {
                    name: value.to_string(),
                    arguments: function_arguments?,
                },
            )?
        }
        Some(Ok(_)) => {
            parse_operation_if_operator_follows(tokens, Node::VariableReference(value.to_string()))?
        }
        Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_identifier() {
        use crate::parsers::identifier_parser::parse_identifier;
        use nilang_types::{
            nodes::Node,
            tokens::{Token, TokenType},
        };
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
