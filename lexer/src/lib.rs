use errors::NilangError;
use nilang_types::tokens::Token;
use tokenizer::Tokenizer;

mod tokenizer;

pub fn lex(input: &str) -> impl Iterator<Item = Result<Token, NilangError>> + '_ {
    Tokenizer::new(input)
}

#[cfg(test)]
mod tests;
