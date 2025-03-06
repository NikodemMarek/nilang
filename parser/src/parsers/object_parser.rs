use std::collections::HashMap;

use errors::ParserErrors;
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse_expression;

pub fn parse_object<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<HashMap<Box<str>, ExpressionNode>, ParserErrors> {
    tokens.assume_opening_brace()?;

    let mut fields = HashMap::new();

    loop {
        match tokens.assume_next()? {
            Token {
                token: TokenType::Identifier(name),
                ..
            } => {
                tokens.assume_colon()?;

                fields.insert(name, parse_expression(tokens)?);

                match tokens.assume_next()? {
                    Token {
                        token: TokenType::Comma,
                        ..
                    } => {}
                    Token {
                        token: TokenType::ClosingBrace,
                        ..
                    } => {
                        break;
                    }
                    Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Comma, TokenType::ClosingBrace]),
                        loc: start,
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingBrace,
                ..
            } => break,
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Identifier("".into()), TokenType::ClosingBrace]),
                loc: start,
            })?,
        }
    }

    Ok(fields)
}
