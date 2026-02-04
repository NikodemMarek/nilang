use errors::NilangError;
use nilang_types::{
    nodes::statements::StatementNode,
    tokens::{Keyword, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type_annotation};

pub fn parse_variable_declaration<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::Variable)?;

    let (_, _, name) = tokens.assume_identifier()?;

    let r#type = parse_type_annotation(tokens)?;

    tokens.assume(TokenType::Equals)?;

    let value = parse_expression(tokens)?;

    tokens.assume(TokenType::Semicolon)?;

    Ok(StatementNode::VariableDeclaration {
        name,
        r#type,
        value: Box::new(value),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{expressions::ExpressionNode, statements::StatementNode, Type},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::{
        multi_peekable::MultiPeekable,
        parsers::variable_declaration_parser::parse_variable_declaration,
    };

    #[test]
    fn test_parse_variable_declaration() {
        assert_eq!(
            parse_variable_declaration(&mut MultiPeekable::new(
                [
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
            ),)
            .unwrap(),
            StatementNode::VariableDeclaration {
                name: "test".into(),
                r#type: Type::Int,
                value: Box::new(ExpressionNode::Number(9.))
            }
        );

        assert_eq!(
            parse_variable_declaration(&mut MultiPeekable::new(
                [
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
            ),)
            .unwrap(),
            StatementNode::VariableDeclaration {
                name: "test".into(),
                r#type: Type::Int,
                value: Box::new(ExpressionNode::VariableReference("test2".into()))
            }
        );
    }
}
