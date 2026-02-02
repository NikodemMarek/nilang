use errors::NilangError;
use nilang_types::{nodes::StatementNode, tokens::Keyword};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, scope_parser::parse_scope};

pub fn parse_while_loop<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::While)?;

    let condition = parse_expression(tokens)?;
    let body = parse_scope(tokens)?;

    Ok(StatementNode::WhileLoop { condition, body })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, StatementNode},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::parsers::while_loop_parser::parse_while_loop;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            parse_while_loop(
                &mut [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::While),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("true".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 5),
                        end: (0, 5),
                    }),
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            StatementNode::WhileLoop {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
            }
        );
    }
}
