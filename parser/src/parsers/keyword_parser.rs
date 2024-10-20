use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::{
    function_declaration_parser::parse_function_declaration, return_parser::parse_return,
    variable_declaration_parser::parse_variable_declaration,
};

pub fn parse_keyword<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    let Token { value, .. } = tokens.peek().unwrap().as_ref().unwrap().clone();

    Ok(match value.as_str() {
        "rt" => parse_return(tokens)?,
        "fn" => parse_function_declaration(tokens)?,
        "vr" => parse_variable_declaration(tokens)?,
        _ => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([TokenType::Keyword]),
            loc: (0, 1),
        })?,
    })
}

#[cfg(test)]
mod tests {
    use crate::parsers::keyword_parser::parse_keyword;
    use nilang_types::{
        nodes::Node,
        tokens::{Token, TokenType},
    };

    #[test]
    fn test_parse_keyword() {
        assert_eq!(
            parse_keyword(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword,
                        value: "rt".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "5".to_string(),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
                        token: TokenType::Keyword,
                        value: "fn".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier,
                        value: "test".to_string(),
                        start: (0, 3),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        value: "(".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        value: ")".to_string(),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        value: "{".to_string(),
                        start: (0, 9),
                        end: (0, 9),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        value: "}".to_string(),
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
                        token: TokenType::Keyword,
                        value: "vr".to_string(),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier,
                        value: "test".to_string(),
                        start: (0, 3),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        value: "=".to_string(),
                        start: (0, 7),
                        end: (0, 7),
                    }),
                    Ok(Token {
                        token: TokenType::Literal,
                        value: "6".to_string(),
                        start: (0, 8),
                        end: (0, 8),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        value: ";".to_string(),
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
