use std::iter::Peekable;

use errors::ParserErrors;
use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

pub fn parse_literal<'a, I>(
    tokens: &mut Peekable<I>,
    Token { token, value, .. }: &Token,
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    if let TokenType::Literal = token {
        Ok(match tokens.peek() {
            Some(Token {
                token: TokenType::OpeningParenthesis,
                ..
            }) => {
                // Function call
                todo!()
            }
            _ => Node::VariableReference(value.to_owned()),
        })
    } else {
        Err(ParserErrors::ThisNeverHappens)?
    }
}
