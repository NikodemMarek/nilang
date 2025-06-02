use std::usize;

use assuming_iterator::PeekableAssumingIterator;
use errors::{NilangError, ParserErrors};
use nilang_types::{
    nodes::{FunctionDeclaration, StructureDeclaration},
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

mod assuming_iterator;
mod parsers;

pub fn parse(
    tokens: impl Iterator<Item = Result<Localizable<TokenType>, NilangError>>,
) -> Result<(Vec<FunctionDeclaration>, Vec<StructureDeclaration>), NilangError> {
    let mut tokens = tokens.peekable();

    let mut structures = Vec::new();
    let mut functions = Vec::new();
    while tokens.peek().is_some() {
        if let TokenType::Keyword(value) = &tokens.peek_valid()?.object {
            match value {
                Keyword::Function => {
                    let function = parsers::function_definition_parser::parse_function_definition(
                        &mut tokens,
                    )?;
                    functions.push(function);
                }
                Keyword::Structure => {
                    let structure = parsers::structure_parser::parse_structure(&mut tokens)?;
                    structures.push(structure);
                }
                Keyword::Return | Keyword::Variable => {
                    return Err(NilangError {
                        location: Location::at(usize::MAX, usize::MAX),
                        error: ParserErrors::ExpectedTokens(
                            [
                                TokenType::Keyword(Keyword::Structure),
                                TokenType::Keyword(Keyword::Function),
                            ]
                            .to_vec(),
                        )
                        .into(),
                    });
                }
            }
        } else {
            return Err(NilangError {
                location: Location::at(usize::MAX, usize::MAX),
                error: ParserErrors::ExpectedTokens(
                    [
                        TokenType::Keyword(Keyword::Structure),
                        TokenType::Keyword(Keyword::Function),
                    ]
                    .to_vec(),
                )
                .into(),
            });
        }
    }

    Ok((functions, structures))
}
