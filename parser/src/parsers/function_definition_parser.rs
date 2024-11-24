use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Keyword, Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parameter_list_parser::parse_parameter_list, parse};

pub fn parse_function_definition<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Node, ParserErrors> {
    tokens.assume_keyword(Keyword::Function)?;

    let (_, _, name) = tokens.assume_identifier()?;

    let parameters = parse_parameter_list(tokens)?;

    tokens.assume_opening_brace()?;

    let body = {
        let mut program = Vec::new();

        loop {
            match tokens.peek_valid()? {
                Token {
                    token: TokenType::ClosingBrace,
                    ..
                } => {
                    tokens.next();
                    break;
                }
                Token { .. } => {
                    program.push(parse(tokens)?);
                }
            }
        }

        program
    };

    Ok(Node::FunctionDeclaration {
        name: name.to_string(),
        parameters,
        body: Box::new(Node::Scope(body)),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Node,
        tokens::{Keyword, Token, TokenType},
    };

    use super::parse_function_definition;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            &parse_function_definition(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Function),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("main".into()),
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
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 11),
                        end: (0, 12),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 14),
                        end: (0, 14),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 15),
                        end: (0, 15),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 16),
                        end: (0, 16),
                    })
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            &Node::FunctionDeclaration {
                name: "main".to_string(),
                parameters: Vec::new(),
                body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                    Node::Number(6.)
                ))]))),
            }
        );
    }
}
