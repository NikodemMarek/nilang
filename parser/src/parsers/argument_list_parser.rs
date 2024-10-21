use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    identifier_parser::parse_identifier, literal_parser::parse_literal,
    operation_parser::parse_operation_if_operator_follows, parenthesis_parser::parse_parenthesis,
};

pub fn parse_argument_list<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Vec<Node>, ParserErrors> {
    tokens.assume_opening_parenthesis()?;

    Ok({
        let mut arguments = Vec::new();

        loop {
            match tokens.peek_valid()? {
                Token {
                    token: TokenType::Literal(_),
                    ..
                } => {
                    let literal = parse_literal(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, literal?)?);

                    match tokens.assume_next()? {
                        Token {
                            token: TokenType::ClosingParenthesis,
                            ..
                        } => break,
                        Token {
                            token: TokenType::Comma,
                            ..
                        } => {}
                        Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                            tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                            loc: start,
                        })?,
                    }
                }
                Token {
                    token: TokenType::Identifier(_),
                    ..
                } => {
                    let identifier = parse_identifier(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, identifier?)?);

                    match tokens.assume_next()? {
                        Token {
                            token: TokenType::ClosingParenthesis,
                            ..
                        } => break,
                        Token {
                            token: TokenType::Comma,
                            ..
                        } => {}
                        Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                            tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                            loc: start,
                        })?,
                    }
                }
                Token {
                    token: TokenType::OpeningParenthesis,
                    ..
                } => {
                    let parenthesis = parse_parenthesis(tokens);
                    arguments.push(parse_operation_if_operator_follows(tokens, parenthesis?)?);

                    match tokens.assume_next()? {
                        Token {
                            token: TokenType::ClosingParenthesis,
                            ..
                        } => break,
                        Token {
                            token: TokenType::Comma,
                            ..
                        } => {}
                        Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                            tokens: Vec::from([TokenType::Comma, TokenType::ClosingParenthesis]),
                            loc: start,
                        })?,
                    }
                }
                Token {
                    token: TokenType::ClosingParenthesis,
                    ..
                } => {
                    tokens.next();
                    break;
                }
                Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                    tokens: Vec::from([
                        TokenType::Identifier("".into()),
                        TokenType::Literal("".into()),
                        TokenType::OpeningParenthesis,
                        TokenType::ClosingParenthesis,
                    ]),
                    loc: *start,
                })?,
            }
        }

        arguments
    })
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::argument_list_parser::parse_argument_list;

    #[test]
    fn test_parse_argument_list() {
        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("5".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            vec![Node::Number(5.), Node::VariableReference("x".to_string()),]
        );

        assert_eq!(
            parse_argument_list(
                &mut [
                    Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start: (0, 0),
                        end: (0, 0),
                    }),
                    Ok(Token {
                        token: TokenType::Identifier("x".into()),
                        start: (0, 1),
                        end: (0, 1),
                    }),
                    Ok(Token {
                        token: TokenType::Operator(Operator::Add),
                        start: (0, 2),
                        end: (0, 2),
                    }),
                    Ok(Token {
                        token: TokenType::Literal("4".into()),
                        start: (0, 3),
                        end: (0, 3),
                    }),
                    Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start: (0, 4),
                        end: (0, 4),
                    }),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Vec::from([Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::VariableReference("x".to_string())),
                b: Box::new(Node::Number(4.)),
            }])
        );
    }
}
