use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, UNEXPECTED_ERROR};

use super::{
    function_declaration_parser::parse_function_declaration, return_parser::parse_return,
    variable_declaration_parser::parse_variable_declaration,
};

pub fn parse_keyword<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    tkn @ Token { token, value, .. }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    if let TokenType::Keyword = token {
        match value.as_str() {
            "rt" => parse_return(program, tokens, tkn),
            "fn" => parse_function_declaration(tokens, tkn),
            "vr" => parse_variable_declaration(program, tokens, tkn),
            _ => panic!("{}", UNEXPECTED_ERROR),
        }
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}
