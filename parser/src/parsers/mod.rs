use errors::{NilangError, ParserErrors};
use function_call_parser::parse_function_call_statement;
use identifier_parser::parse_identifier;
use literal_parser::parse_literal;
use nilang_types::{
    nodes::{ExpressionNode, StatementNode},
    tokens::{Keyword, TokenType},
    Localizable as L,
};

use operation_parser::parse_operation_if_operator_follows;
use parenthesis_parser::parse_parenthesis;
use return_parser::parse_return;
use variable_declaration_parser::parse_variable_declaration;

use crate::assuming_iterator::PeekableAssumingIterator;

mod arguments_parser;
mod field_access_parser;
mod function_call_parser;
pub mod function_definition_parser;
mod identifier_parser;
mod literal_parser;
mod object_fields_parser;
mod object_parser;
mod operation_parser;
mod parameters_parser;
mod parenthesis_parser;
mod return_parser;
pub mod structure_parser;
mod type_annotation_parser;
mod typed_identifier_list_parser;
mod variable_declaration_parser;

pub fn parse_statement<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<StatementNode>, NilangError> {
    let peek_valid = tokens.peek_valid()?;

    Ok(match &peek_valid.object {
        TokenType::Keyword(value) => match value {
            Keyword::Variable => parse_variable_declaration(tokens)?,
            Keyword::Return => parse_return(tokens)?,
            Keyword::Function | Keyword::Structure => {
                return Err(NilangError {
                    location: peek_valid.location,
                    error: ParserErrors::UnexpectedToken(peek_valid.object.clone()).into(),
                })
            }
        },
        TokenType::Identifier(_) => {
            let name = tokens.assume_identifier()?;
            let function_call = parse_function_call_statement(tokens, name)?;
            tokens.assume(TokenType::Semicolon)?;
            function_call
        }
        TokenType::Operator(_)
        | TokenType::ClosingParenthesis
        | TokenType::ClosingBrace
        | TokenType::OpeningBrace
        | TokenType::Literal(_)
        | TokenType::OpeningParenthesis
        | TokenType::Equals
        | TokenType::Semicolon
        | TokenType::Colon
        | TokenType::Comma
        | TokenType::Dot => Err(NilangError {
            location: peek_valid.location,
            error: ParserErrors::UnexpectedToken(peek_valid.object.clone()).into(),
        })?,
    })
}

pub fn parse_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<ExpressionNode>, NilangError> {
    let expression_node = parse_single_expression(tokens)?;
    parse_operation_if_operator_follows(tokens, expression_node)
}

pub fn parse_single_expression<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<ExpressionNode>, NilangError> {
    let peek_valid = tokens.peek_valid()?;

    Ok(match peek_valid.object {
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
        | TokenType::Dot => Err(NilangError {
            location: peek_valid.location,
            error: ParserErrors::UnexpectedToken(peek_valid.object.clone()).into(),
        })?,
    })
}
