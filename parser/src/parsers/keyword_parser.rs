use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Keyword, Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    function_definition_parser::parse_function_definition, return_parser::parse_return,
    variable_declaration_parser::parse_variable_declaration,
};

pub fn parse_keyword<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    let value = if let Token {
        token: TokenType::Keyword(value),
        ..
    } = tokens.peek_valid()?
    {
        value
    } else {
        unreachable!()
    };

    Ok(match *value {
        Keyword::Variable => parse_variable_declaration(tokens)?,
        Keyword::Function => parse_function_definition(tokens)?,
        Keyword::Return => parse_return(tokens)?,
    })
}

#[cfg(test)]
mod tests {
    use crate::parsers::keyword_parser::parse_keyword;
    use nilang_types::{
        nodes::Node,
        tokens::{Keyword, Token, TokenType},
    };

    #[test]
    fn test_parse_keyword() {
        assert_eq!(
            parse_keyword(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 4),
                        end: (0, 4),
                    })
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Return(Box::new(Node::Number(5.)))
        );

        assert_eq!(
            parse_keyword(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Function),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 3),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 10),
                        end: (0, 10),
                    })
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::FunctionDeclaration {
                name: "test".to_string(),
                parameters: Vec::new(),
                body: Box::new(Node::Scope(Vec::new())),
            }
        );

        assert_eq!(
            parse_keyword(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Variable),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("test".into()),
                        start: (0, 3),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,

                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 9),
                        end: (0, 9),
                    })
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::VariableDeclaration {
                name: "test".to_string(),
                value: Box::new(Node::Number(6.)),
            }
        );
    }
}
