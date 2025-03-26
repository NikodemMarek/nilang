use aggregated_iterator::Tokenizer;
use errors::LexerErrors;
use nilang_types::tokens::Token;

mod aggregated_iterator;

pub fn lex(input: &str) -> impl Iterator<Item = Result<Token, LexerErrors>> + '_ {
    Tokenizer::new(input)
}

#[cfg(test)]
mod tests;
