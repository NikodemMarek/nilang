use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::parse;

pub fn parse_scope<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    let start = tokens.assume_opening_brace()?;

    let mut in_scope = Vec::new();
    while let token = tokens.peek_valid()? {
        if let Token {
            token: TokenType::ClosingBrace,
            end,
            ..
        } = token
        {
            if in_scope.is_empty() {
                Err(ParserErrors::EmptyScope {
                    from: start,
                    to: *end,
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
