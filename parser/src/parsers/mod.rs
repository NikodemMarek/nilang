use std::iter::Peekable;

use errors::ParserErrors;
use identifier_parser::parse_identifier;
use keyword_parser::parse_keyword;
use literal_parser::parse_literal;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};
use operation_parser::parse_operation_greedy;
use parenthesis_parser::parse_parenthesis;
use scope_parser::parse_scope;

pub mod function_declaration_parser;
pub mod identifier_parser;
pub mod keyword_parser;
pub mod literal_parser;
pub mod operation_parser;
pub mod parenthesis_parser;
pub mod return_parser;
pub mod scope_parser;
pub mod variable_declaration_parser;

pub fn parse<'a, I>(program: &mut Vec<Node>, tokens: &mut Peekable<I>) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(
        tkn @ Token {
            token, start, end, ..
        },
    ) = tokens.next()
    {
        Ok(match token {
            TokenType::Literal => parse_literal(tkn)?,
            TokenType::Operator => parse_operation_greedy(program, tokens, tkn)?,
            TokenType::OpeningParenthesis => parse_parenthesis(tokens, (start, end))?,
            TokenType::OpeningBrace => parse_scope(tokens)?,
            TokenType::Keyword => parse_keyword(program, tokens, tkn)?,
            TokenType::Identifier => parse_identifier(tokens, tkn)?,
            token @ TokenType::ClosingParenthesis
            | token @ TokenType::ClosingBrace
            | token @ TokenType::Equals
            | token @ TokenType::Semicolon
            | token @ TokenType::Comma => Err(ParserErrors::UnexpectedToken {
                token: *token,
                loc: *start,
            })?,
        })
    } else {
        Err(ParserErrors::EndOfInput {
            loc: (usize::MAX, usize::MAX),
        })?
    }
}
