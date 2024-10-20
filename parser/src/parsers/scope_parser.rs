use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::parse;

pub fn parse_scope<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    let scope_start = match tokens.peek() {
        Some(Ok(Token { start, .. })) => (start.0, start.1 - 1),
        Some(_) | None => unreachable!(),
    };

    let mut in_scope = Vec::new();
    while let Some(Ok(token)) = tokens.peek() {
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
            let node = parse(tokens)?;
            in_scope.push(node);
        }
    }

    Ok(Node::Scope(in_scope))
}
