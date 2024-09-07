use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

use super::parse;

pub fn parse_scope<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    let mut in_scope = Vec::new();

    while let Some(token) = tokens.peek() {
        if token.token == TokenType::ClosingBrace {
            tokens.next();
            break;
        } else {
            let node = parse(&mut in_scope, tokens);
            in_scope.push(node);
        }
    }

    Node::Scope(in_scope)
}
