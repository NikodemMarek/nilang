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

    use crate::{multi_peekable::MultiPeekable, parsers::while_loop_parser::parse_while_loop};

    #[test]
    fn test_parse_while_loop() {
        assert_eq!(
            parse_while_loop(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::While),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("true".into()),
                        start: (0, 2),
                        end: (0, 5),
                    }),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 6),
                        end: (0, 6),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (0, 7),
                        end: (0, 7),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            StatementNode::WhileLoop {
                condition: ExpressionNode::Boolean(true),
                body: Box::new([]),
            }
        );
    }
}
