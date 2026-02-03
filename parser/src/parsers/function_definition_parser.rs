use errors::NilangError;
use nilang_types::{nodes::FunctionDeclaration, tokens::Keyword};

use crate::{assuming_iterator::PeekableAssumingIterator, parsers::scope_parser::parse_scope};

use super::{
    parameter_list_parser::parse_parameter_list, type_annotation_parser::parse_type_annotation,
};

pub fn parse_function_definition<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<FunctionDeclaration, NilangError> {
    tokens.assume_keyword(Keyword::Function)?;

    let (_, _, name) = tokens.assume_identifier()?;
    let parameters = parse_parameter_list(tokens)?;
    let return_type = parse_type_annotation(tokens)?;
    let body = parse_scope(tokens)?;

    Ok(FunctionDeclaration {
        name,
        parameters,
        return_type,
        body,
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionDeclaration, StatementNode, Type},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::multi_peekable::MultiPeekable;

    use super::parse_function_definition;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            parse_function_definition(&mut MultiPeekable::new(
                [
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
            ),)
            .unwrap(),
            FunctionDeclaration {
                name: "main".into(),
                parameters: [].into(),
                return_type: Type::Int,
                body: Box::new([StatementNode::Return(Box::new(ExpressionNode::Number(6.)))]),
            }
        );
    }
}
