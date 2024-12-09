use errors::ParserErrors;
use nilang_types::{nodes::Node, tokens::Keyword};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    type_annotation_parser::parse_type_annotation, value_yielding_parser::parse_value_yielding,
};

pub fn parse_variable_declaration<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Node, ParserErrors> {
    tokens.assume_keyword(Keyword::Variable)?;

    let (_, _, name) = tokens.assume_identifier()?;

    let r#type = parse_type_annotation(tokens)?;

    tokens.assume_equals()?;

    let value = parse_value_yielding(tokens)?;

    tokens.assume_semicolon()?;

    Ok(Node::VariableDeclaration {
        name,
        r#type,
        value: Box::new(value),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::parsers::variable_declaration_parser::parse_variable_declaration;

    #[test]
    fn test_parse_variable_declaration() {
        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 6),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 11),
                        end: (0, 11),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".into(),
                r#type: "int".into(),
                value: Box::new(Node::Number(9.))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 6),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 10),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".into(),
                r#type: "int".into(),
                value: Box::new(Node::VariableReference("test2".into()))
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 6),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 11),
                        end: (0, 11),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 12),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 13),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".into(),
                r#type: "int".into(),
                value: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                })
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 6),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 10),
                        end: (0, 10),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test2".into()),
                        start: (0, 11),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 16),
                        end: (0, 16),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 17),
                        end: (0, 17),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 18),
                        end: (0, 18),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 19),
                        end: (0, 19),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".into(),
                r#type: "int".into(),
                value: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::VariableReference("test2".into())),
                    b: Box::new(Node::Number(9.)),
                })
            }
        );

        assert_eq!(
            parse_variable_declaration(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 1),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 6),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("abc".into()),
                        start: (0, 10),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 13),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 15),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("9".into()),
                        start: (0, 16),
                        end: (0, 16),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 17),
                        end: (0, 17),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 18),
                        end: (0, 18),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".into(),
                r#type: "int".into(),
                value: Box::new(Node::FunctionCall {
                    name: "abc".into(),
                    arguments: [Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(6.)),
                        b: Box::new(Node::Number(9.)),
                    }]
                    .into()
                })
            }
        );
    }
}
