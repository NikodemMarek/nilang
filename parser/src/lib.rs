use std::usize;

use assuming_iterator::PeekableAssumingIterator;
use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::{FunctionDeclaration, StructureDeclaration},
    tokens::{Keyword, Token, TokenType},
};

mod assuming_iterator;
mod parsers;

pub fn parse(
    tokens: impl Iterator<Item = Result<Token, NilangError>>,
) -> Result<(Vec<FunctionDeclaration>, Vec<StructureDeclaration>), NilangError> {
    let mut tokens = tokens.peekable();

    let mut structures = Vec::new();
    let mut functions = Vec::new();
    while tokens.peek().is_some() {
        if let TokenType::Keyword(value) = &tokens.peek_valid()?.token {
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
                Keyword::Return | Keyword::Variable | Keyword::If => {
                    return Err(NilangError {
                        location: CodeLocation::at(usize::MAX, usize::MAX),
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
                location: CodeLocation::at(usize::MAX, usize::MAX),
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
