use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, UNEXPECTED_ERROR};

pub fn parse_literal<'a, I>(tokens: &mut Peekable<I>, Token { token, value, .. }: &Token) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    if let TokenType::Literal = token {
        match tokens.peek() {
            Some(Token {
                token: TokenType::OpeningParenthesis,
                ..
            }) => {
                // Function call
                todo!()
            }
            _ => Node::VariableReference(value.to_owned()),
        }
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}
