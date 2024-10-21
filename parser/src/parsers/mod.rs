use errors::ParserErrors;
use identifier_parser::parse_identifier;
use keyword_parser::parse_keyword;
use literal_parser::parse_literal;
use nilang_types::{nodes::Node, tokens::TokenType};

use parenthesis_parser::parse_parenthesis;
use scope_parser::parse_scope;

use crate::assuming_iterator::PeekableAssumingIterator;

pub mod argument_list_parser;
pub mod function_definition_parser;
pub mod identifier_parser;
pub mod keyword_parser;
pub mod literal_parser;
pub mod operation_parser;
pub mod parameter_list_parser;
pub mod parenthesis_parser;
pub mod return_parser;
pub mod scope_parser;
pub mod variable_declaration_parser;

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
        | TokenType::Comma => Err(ParserErrors::UnexpectedToken {
            token: peek_valid.token.clone(),
            loc: peek_valid.start,
        })?,
    })
}
