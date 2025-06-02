use errors::NilangError;
use nilang_types::{tokens::TokenType, Localizable};
use tokenizer::Tokenizer;

mod tokenizer;

pub fn lex(input: &str) -> impl Iterator<Item = Result<Localizable<TokenType>, NilangError>> + '_ {
    Tokenizer::new(input)
}

#[cfg(test)]
mod tests;
