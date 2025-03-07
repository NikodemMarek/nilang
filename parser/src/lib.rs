use std::{collections::HashMap, usize};

use assuming_iterator::PeekableAssumingIterator;
use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Program,
    tokens::{Keyword, Token, TokenType},
};

mod assuming_iterator;
mod parsers;

pub fn parse(
    tokens: impl Iterator<Item = Result<Token, LexerErrors>>,
) -> Result<Program, ParserErrors> {
    let mut tokens = tokens.peekable();

    let mut structures = HashMap::new();
    let mut functions = HashMap::new();
    while tokens.peek().is_some() {
        if let TokenType::Keyword(value) = &tokens.peek_valid()?.token {
            match value {
                Keyword::Function => {
                    let function = parsers::function_definition_parser::parse_function_definition(
                        &mut tokens,
                    )?;
                    functions.insert(function.name.clone(), function);
                }
                Keyword::Structure => {
                    let structure = parsers::structure_parser::parse_structure(&mut tokens)?;
                    structures.insert(structure.name.clone(), structure);
                }
                Keyword::Return | Keyword::Variable => {
                    return Err(ParserErrors::ExpectedTokens {
                        tokens: [
                            TokenType::Keyword(Keyword::Structure),
                            TokenType::Keyword(Keyword::Function),
                        ]
                        .to_vec(),
                        loc: (usize::MAX, usize::MAX),
                    });
                }
            }
        } else {
            return Err(ParserErrors::ExpectedTokens {
                tokens: [
                    TokenType::Keyword(Keyword::Structure),
                    TokenType::Keyword(Keyword::Function),
                ]
                .to_vec(),
                loc: (usize::MAX, usize::MAX),
            });
        }
    }

    Ok(Program {
        structures,
        functions,
    })
}
