use std::collections::HashMap;

use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{parse_expression, type_annotation_parser::parse_type};

pub fn parse_object<I: PeekableAssumingIterator>(
    tokens: &mut I,
    name: Box<str>,
) -> Result<ExpressionNode, NilangError> {
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
                    return Err(NilangError {
                        location: CodeLocation::range(start.0, start.1, end.0, end.1),
                        error: ParserErrors::DuplicateField(name).into(),
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
                    Token { start, .. } => Err(NilangError {
                        location: CodeLocation::at(start.0, start.1),
                        error: ParserErrors::ExpectedTokens(Vec::from([
                            TokenType::Comma,
                            TokenType::ClosingBrace,
                        ]))
                        .into(),
                    })?,
                }
            }
            Token {
                token: TokenType::ClosingBrace,
                ..
            } => break,
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([
                    TokenType::Identifier("".into()),
                    TokenType::ClosingBrace,
                ]))
                .into(),
            })?,
        }
    }

    Ok(ExpressionNode::Object {
        r#type: parse_type(&name),
        fields,
    })
}
