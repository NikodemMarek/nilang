use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

pub fn parse_literal<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    let (value, start, end) = if let Some(Ok(Token {
        token: TokenType::Literal(value),
        start,
        end,
        ..
    })) = tokens.next()
    {
        (value, start, end)
    } else {
        unreachable!()
    };

    Ok(match value.parse() {
        Ok(parsed) => Node::Number(parsed),
        Err(_) => Err(ParserErrors::NotANumber {
            from: start,
            to: end,
        })?,
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Node,
        tokens::{Token, TokenType},
    };

    use crate::parsers::literal_parser::parse_literal;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("54".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Number(54.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("6.".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Number(6.)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal(".2".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Number(0.2)
        );
        assert_eq!(
            parse_literal(
                &mut [Ok(Token {
                    token: TokenType::Literal("8.5".into()),
                    start: (0, 0),
                    end: (0, 2),
                })]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Node::Number(8.5)
        );
    }
}
