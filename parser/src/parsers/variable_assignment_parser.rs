use errors::NilangError;
use nilang_types::{nodes::StatementNode, tokens::TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_variable_assignment<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    let (_, _, name) = tokens.assume_identifier()?;

    tokens.assume(TokenType::Equals)?;

    let value = parse_expression(tokens)?;

    tokens.assume(TokenType::Semicolon)?;

    Ok(StatementNode::VariableAssignment {
        name,
        value: value.into(),
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, StatementNode},
        tokens::{Token, TokenType},
    };

    use crate::{
        multi_peekable::MultiPeekable,
        parsers::variable_assignment_parser::parse_variable_assignment,
    };

    #[test]
    fn test_parse_variable_assignment() {
        assert_eq!(
            parse_variable_assignment(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Equals,
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("true".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                ]
                .into_iter()
            ))
            .unwrap(),
            StatementNode::VariableAssignment {
                name: "x".into(),
                value: Box::new(ExpressionNode::Boolean(true))
            }
        );
    }
}
