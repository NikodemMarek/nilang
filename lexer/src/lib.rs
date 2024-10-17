#![feature(iterator_try_collect)]

use aggregated_iterator::AggregatedIterator;
use nilang_types::tokens::Token;

mod aggregated_iterator;

pub fn lex(input: &str) -> eyre::Result<Vec<Token>> {
    let mut aggregated_iterator = AggregatedIterator::new(input);

    Ok(aggregated_iterator.try_collect()?)
}

#[cfg(test)]
mod tests;
