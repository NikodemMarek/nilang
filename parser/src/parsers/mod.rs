use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use identifier_parser::parse_identifier;
use keyword_parser::parse_keyword;
use literal_parser::parse_literal;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use parenthesis_parser::parse_parenthesis;
use scope_parser::parse_scope;

pub mod function_arguments_parser;
pub mod function_declaration_parser;
pub mod identifier_parser;
pub mod keyword_parser;
pub mod literal_parser;
pub mod operation_parser;
pub mod parenthesis_parser;
pub mod return_parser;
pub mod scope_parser;
pub mod variable_declaration_parser;

pub fn parse<I>(tokens: &mut Peekable<I>) -> Result<Node, ParserErrors>
where
    I: Iterator<Item = Result<Token, LexerErrors>>,
{
    Ok(match tokens.peek() {
        Some(Ok(Token { token, start, .. })) => match token {
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
                token: token.clone(),
                loc: *start,
            })?,
        },
        Some(Err(e)) => Err(ParserErrors::LexerError(e.clone()))?,
        None => Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?,
    })
}
