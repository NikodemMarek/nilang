use std::iter::Peekable;

use keyword_parser::parse_keyword;
use literal_parser::parse_literal;
use nilang_lexer::tokens::{Token, TokenType};
use number_parser::parse_number;
use operation_parser::parse_operation;
use parenthesis_parser::parse_parenthesis;
use scope_parser::parse_scope;

use crate::{nodes::Node, UNEXPECTED_END_OF_INPUT_ERROR};

pub mod function_declaration_parser;
pub mod keyword_parser;
pub mod literal_parser;
pub mod number_parser;
pub mod operation_parser;
pub mod parenthesis_parser;
pub mod scope_parser;
pub mod variable_declaration_parser;

pub fn parse<'a, I>(program: &mut Vec<Node>, tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(
        tkn @ Token {
            token, start, end, ..
        },
    ) = tokens.next()
    {
        match token {
            TokenType::Number => parse_number(tkn),
            TokenType::Operator => parse_operation(program, tokens, tkn),
            TokenType::OpeningParenthesis => parse_parenthesis(tokens, (start, end)),
            TokenType::ClosingParenthesis => panic!("[{}] Unexpected closing parenthesis", start),
            TokenType::OpeningBrace => parse_scope(tokens),
            TokenType::ClosingBrace => panic!("[{}] Unexpected closing brace", start),
            TokenType::Keyword => parse_keyword(program, tokens, tkn),
            TokenType::Equals => panic!("[{}] Unexpected equals sign", start),
            TokenType::Literal => parse_literal(tokens, tkn),
            TokenType::Semicolon => panic!("[{}] Unexpected semicolon", start),
        }
    } else {
        panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
    }
}
