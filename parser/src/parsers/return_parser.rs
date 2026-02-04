use errors::NilangError;
use nilang_types::{
    nodes::statements::StatementNode,
    tokens::{Keyword, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_return<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, NilangError> {
    tokens.assume_keyword(Keyword::Return)?;

    let value = parse_expression(tokens)?;

    tokens.assume(TokenType::Semicolon)?;

    Ok(StatementNode::Return(Box::new(value)))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{expressions::ExpressionNode, statements::StatementNode},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::return_parser::parse_return};

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Return),
                        start: (0, 0),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("6".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::Semicolon,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
            ),)
            .unwrap(),
            StatementNode::Return(Box::new(ExpressionNode::Number(6.)))
        );
    }
}
