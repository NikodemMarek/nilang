use errors::ParserErrors;
use nilang_types::{
    nodes::FunctionDeclaration,
    tokens::{Keyword, Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    parameter_list_parser::parse_parameter_list, parse_statement,
    type_annotation_parser::parse_type_annotation,
};

pub fn parse_function_definition<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<FunctionDeclaration, ParserErrors> {
    tokens.assume_keyword(Keyword::Function)?;

    let (_, _, name) = tokens.assume_identifier()?;

    let parameters = parse_parameter_list(tokens)?;

    let return_type = parse_type_annotation(tokens)?;

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
                    program.push(parse_statement(tokens)?);
                }
            }
        }

        program
    };

    Ok(FunctionDeclaration {
        name,
        parameters,
        return_type,
        body: body.into(),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionDeclaration, StatementNode},
        tokens::{Keyword, Token, TokenType},
    };

    use super::parse_function_definition;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            parse_function_definition(
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
                        token: TokenType::Colon,
                        start: (0, 9),
                        end: (0, 9)
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (0, 10),
                        end: (0, 12)
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 13),
                        end: (0, 13),
                    }),
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 14),
                        end: (0, 15),
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
            FunctionDeclaration {
                name: "main".into(),
                parameters: [].into(),
                return_type: "int".into(),
                body: Box::new([StatementNode::Return(Box::new(ExpressionNode::Number(6.)))]),
            }
        );
    }
}
