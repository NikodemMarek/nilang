use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows, parenthesis_parser::parse_parenthesis,
};

pub fn parse_value_yielding<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Node, ParserErrors> {
    let node = match tokens.peek_valid()? {
        Token {
            token: TokenType::Literal(_),
            ..
        } => parse_literal(tokens),
        Token {
            token: TokenType::Identifier(_),
            ..
        } => parse_identifier(tokens),
        Token {
            token: TokenType::OpeningParenthesis,
            ..
        } => parse_parenthesis(tokens),
        Token { end, .. } => Err(ParserErrors::ExpectedTokens {
            tokens: Vec::from([
                TokenType::Literal("".into()),
                TokenType::Identifier("".into()),
                TokenType::OpeningParenthesis,
            ]),
            loc: *end,
        }),
    }?;

    parse_operation_if_operator_follows(tokens, node)
}
