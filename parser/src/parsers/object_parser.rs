use std::collections::HashMap;

use errors::ParserErrors;
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type};

pub fn parse_object<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Box<str>,
) -> Result<ExpressionNode, ParserErrors> {
    tokens.assume(TokenType::OpeningBrace)?;

    let mut fields = HashMap::new();

    loop {
        match tokens.assume_next()? {
            Token {
                token: TokenType::Identifier(name),
                start,
                end,
                ..
            } => {
                tokens.assume(TokenType::Colon)?;

                if fields
                    .insert(name.clone(), parse_expression(tokens)?)
                    .is_some()
                {
                    return Err(ParserErrors::DuplicateField {
                        from: start,
                        to: end,
                        name,
                    });
                };

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

    Ok(ExpressionNode::Object {
        r#type: parse_type(&name),
        fields,
    })
}
