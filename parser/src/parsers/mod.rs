use errors::ParserErrors;
use identifier_parser::parse_identifier;
use keyword_parser::parse_keyword;
use literal_parser::parse_literal;
use nilang_types::{nodes::Node, tokens::TokenType};

use parenthesis_parser::parse_parenthesis;
use scope_parser::parse_scope;

use crate::assuming_iterator::PeekableAssumingIterator;

mod argument_list_parser;
mod function_definition_parser;
mod identifier_parser;
mod keyword_parser;
mod literal_parser;
mod object_parser;
mod operation_parser;
mod parameter_list_parser;
mod parenthesis_parser;
mod return_parser;
mod scope_parser;
mod structure_parser;
mod type_annotation_parser;
mod value_yielding_parser;
mod variable_declaration_parser;

pub fn parse<I: PeekableAssumingIterator>(tokens: &mut I) -> Result<Node, ParserErrors> {
    let peek_valid = tokens.peek_valid()?;

    Ok(match peek_valid.token {
        TokenType::Literal(_) => parse_literal(tokens)?,
        TokenType::OpeningParenthesis => parse_parenthesis(tokens)?,
        TokenType::OpeningBrace => parse_scope(tokens)?,
        TokenType::Keyword(_) => parse_keyword(tokens)?,
        TokenType::Identifier(_) => parse_identifier(tokens)?,
        TokenType::Operator(_)
        | TokenType::ClosingParenthesis
        | TokenType::ClosingBrace
        | TokenType::Equals
        | TokenType::Semicolon
        | TokenType::Colon
        | TokenType::Comma
        | TokenType::Dot => Err(ParserErrors::UnexpectedToken {
            token: peek_valid.token.clone(),
            loc: peek_valid.start,
        })?,
    })
}
