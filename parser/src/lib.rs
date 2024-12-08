use std::{collections::HashMap, usize};

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::{Node, Program},
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
        match parsers::parse(&mut tokens)? {
            structure @ Node::Structure { .. } => {
                structures.insert(
                    match structure {
                        Node::Structure { ref name, .. } => name.clone(),
                        _ => unreachable!(),
                    },
                    structure,
                );
            }
            function @ Node::FunctionDeclaration { .. } => {
                functions.insert(
                    match function {
                        Node::FunctionDeclaration { ref name, .. } => name.clone(),
                        _ => unreachable!(),
                    },
                    function,
                );
            }
            _ => {
                return Err(ParserErrors::ExpectedTokens {
                    tokens: [
                        TokenType::Keyword(Keyword::Structure),
                        TokenType::Keyword(Keyword::Function),
                    ]
                    .to_vec(),
                    loc: (usize::MAX, usize::MAX),
                });
            }
        };
    }

    Ok(Program {
        structures,
        functions,
    })
}
