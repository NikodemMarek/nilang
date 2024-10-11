use std::iter::Peekable;

use errors::ParserErrors;
use nilang_lexer::tokens::{Token, TokenType};

use crate::nodes::Node;

use super::parse;

pub fn parse_scope<'a, I>(tokens: &mut Peekable<I>) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    let scope_start = match tokens.peek() {
        Some(Token { start, .. }) => (start.0, start.1 - 1),
        None => Err(ParserErrors::ThisNeverHappens)?,
    };

    let mut in_scope = Vec::new();
    while let Some(token) = tokens.peek() {
        if let Token {
            token: TokenType::ClosingBrace,
            start,
            ..
        } = token
        {
            if in_scope.is_empty() {
                Err(ParserErrors::EmptyScope {
                    from: (scope_start.0, scope_start.1 + 1),
                    to: (start.0, start.1 - 1),
                })?
            }

            tokens.next();
            break;
        } else {
            let node = parse(&mut in_scope, tokens)?;
            in_scope.push(node);
        }
    }

    Ok(Node::Scope(in_scope))
}
