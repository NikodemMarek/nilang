use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    argument_list_parser::parse_argument_list, object_parser::parse_object,
    operation_parser::parse_operation_if_operator_follows,
};

pub fn parse_identifier<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    let (_, _, name) = tokens.assume_identifier()?;

    let variable_reference = match tokens.peek_valid()? {
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => {
            let arguments = parse_argument_list(tokens)?;
            parse_operation_if_operator_follows(tokens, Node::FunctionCall { name, arguments })?
        }
        Token {
            token: TokenType::Operator(_),
            ..
        } => parse_operation_if_operator_follows(tokens, Node::VariableReference(name))?,
        Token {
            token: TokenType::OpeningBrace,
            ..
        } => Node::Object {
            r#type: name,
            fields: parse_object(tokens)?,
        },
        Token { .. } => Node::VariableReference(name),
    };

    Ok(match tokens.peek_valid()? {
        Token {
            token: TokenType::Dot,
            ..
        } => {
            tokens.next();
            Node::FieldAccess {
                structure: Box::new(variable_reference),
                field: {
                    let (_, _, name) = tokens.assume_identifier()?;
                    name
                },
            }
        }
        Token { .. } => variable_reference,
    })
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
            Node::VariableReference("x".into())
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
                name: "x".into(),
                arguments: [].into()
            }
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
                        token: TokenType::Dot,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::FieldAccess {
                structure: Box::new(Node::VariableReference("x".into())),
                field: "test".into()
            }
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
                        token: TokenType::Dot,
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 4),
                        end: (0, 7),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::FieldAccess {
                structure: Box::new(Node::FunctionCall {
                    name: "x".into(),
                    arguments: [].into()
                }),
                field: "test".into()
            }
        );
    }
}
