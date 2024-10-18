use aggregated_iterator::AggregatedIterator;
use errors::LexerErrors;
use nilang_types::tokens::Token;

mod aggregated_iterator;

pub fn lex(input: &str) -> impl Iterator<Item = Result<Token, LexerErrors>> + '_ {
    AggregatedIterator::new(input)
}

#[cfg(test)]
mod tests;
