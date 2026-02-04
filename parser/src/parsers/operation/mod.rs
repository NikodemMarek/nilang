use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::expressions::ExpressionNode,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

mod expression_combinator;
mod operation_extender;
mod precendence;

pub fn lookup_operation_recursive<I: PeekableAssumingIterator>(
    tokens: &mut I,
    preceeding: ExpressionNode,
) -> Result<ExpressionNode, NilangError> {
    if let Token {
        token: TokenType::Operator(_),
        ..
    } = tokens.peek_valid()?
    {
        let (start, end, operator) = tokens.assume_operator()?;
        let following = super::parse_single_expression(tokens)?;

        let expression = expression_combinator::combine_expressions(
            preceeding, operator, following,
        )
        .map_err(|_| NilangError {
            location: CodeLocation::range(start.0, start.1, end.0, end.1),
            error: ParserErrors::InvalidOperand.into(),
        })?;

        lookup_operation_recursive(tokens, expression)
    } else {
        Ok(preceeding)
    }
}

#[cfg(test)]
mod tests {}
