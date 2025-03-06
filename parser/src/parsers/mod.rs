use errors::ParserErrors;
use identifier_parser::parse_identifier;
use literal_parser::parse_literal;
use nilang_types::{
    nodes::{ExpressionNode, StatementNode},
    tokens::{Keyword, TokenType},
};

use operation_parser::parse_operation_if_operator_follows;
use parenthesis_parser::parse_parenthesis;
use return_parser::parse_return;
use variable_declaration_parser::parse_variable_declaration;

use crate::assuming_iterator::PeekableAssumingIterator;

mod argument_list_parser;
pub mod function_definition_parser;
mod identifier_parser;
mod literal_parser;
mod object_parser;
mod operation_parser;
mod parameter_list_parser;
mod parenthesis_parser;
mod return_parser;
pub mod structure_parser;
mod type_annotation_parser;
mod variable_declaration_parser;

pub fn parse_statement<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StatementNode, ParserErrors> {
    let peek_valid = tokens.peek_valid()?;

    Ok(match &peek_valid.token {
        TokenType::Keyword(value) => match value {
            Keyword::Variable => parse_variable_declaration(tokens)?,
            Keyword::Return => parse_return(tokens)?,
            Keyword::Function | Keyword::Structure => {
                panic!("function and structure declarations are not statements")
            }
        },
        TokenType::Operator(_)
        | TokenType::ClosingParenthesis
        | TokenType::ClosingBrace
        | TokenType::OpeningBrace
        | TokenType::Literal(_)
        | TokenType::Identifier(_)
        | TokenType::OpeningParenthesis
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

pub fn parse_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, ParserErrors> {
    let expression_node = parse_single_expression(tokens)?;
    parse_operation_if_operator_follows(tokens, expression_node)
}

pub fn parse_single_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<ExpressionNode, ParserErrors> {
    let peek_valid = tokens.peek_valid()?;

    Ok(match peek_valid.token {
        TokenType::Literal(_) => parse_literal(tokens)?,
        TokenType::OpeningParenthesis => parse_parenthesis(tokens)?,
        TokenType::Identifier(_) => parse_identifier(tokens)?,
        TokenType::Operator(_)
        | TokenType::ClosingParenthesis
        | TokenType::ClosingBrace
        | TokenType::OpeningBrace
        | TokenType::Equals
        | TokenType::Keyword(_)
        | TokenType::Semicolon
        | TokenType::Colon
        | TokenType::Comma
        | TokenType::Dot => Err(ParserErrors::UnexpectedToken {
            token: peek_valid.token.clone(),
            loc: peek_valid.start,
        })?,
    })
}
