use errors::ParserErrors;
use nilang_types::tokens::{Token, TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_parameter_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Vec<Box<str>>, ParserErrors> {
    tokens.assume_opening_parenthesis()?;

    let mut parameters = Vec::new();

    loop {
        match tokens.assume_next()? {
            Token {
                token: TokenType::Identifier(value),
                ..
            } => {
                parameters.push(value.to_owned());

                match tokens.assume_next()? {
                    Token {
                        token: TokenType::ClosingParenthesis,
                        ..
                    } => break,
                    Token {
                        token: TokenType::Comma,
                        ..
                    } => {}
                    Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                        loc: start,
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingParenthesis,
                ..
            } => break,
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingParenthesis,
                ]),
                loc: start,
            })?,
        }
    }

    Ok(parameters)
}
