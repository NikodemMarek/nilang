use std::iter::Peekable;

use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

use super::parse;

pub fn parse_parenthesis<'a, I>(
    tokens: &mut Peekable<I>,
    (start, end): (&(usize, usize), &(usize, usize)),
) -> eyre::Result<Node>
where
    I: Iterator<Item = &'a Token>,
{
    let mut in_parenthesis = Vec::new();

    let mut last_node_end = end;
    while let Some(token) = tokens.peek() {
        last_node_end = end;
        if token.token == TokenType::ClosingParenthesis {
            tokens.next();
            break;
        } else {
            let node = parse(&mut in_parenthesis, tokens)?;
            in_parenthesis.push(node);
        }
    }

    if in_parenthesis.is_empty() {
        Err(ParserErrors::EmptyParenthesis {
            from: *start,
            to: *last_node_end,
        })?
    }
    if in_parenthesis.len() > 1 {
        Err(ParserErrors::InvalidParenthesisContent {
            from: *start,
            to: *last_node_end,
        })?
    }

    match in_parenthesis.first() {
        Some(node) => Ok(node.to_owned()),
        None => Err(ParserErrors::ThisNeverHappens)?,
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{Node, Operator},
        tokens::{Token, TokenType},
    };

    use crate::parsers::parenthesis_parser::parse_parenthesis;

    #[test]
    fn parse_parenthesis_operations() {
        let tokens = [
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 0), &(0, 4))).unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Number(9.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 0), &(0, 4))).unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Number(9.)),
                }),
                b: Box::new(Node::Number(5.)),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 2), &(0, 6))).unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }
        );

        let tokens = [
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 8),
                end: (0, 8),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 0), &(0, 8))).unwrap(),
            Node::Operation {
                operator: Operator::Add,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 2), &(0, 6))).unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Number(6.)),
                b: Box::new(Node::Operation {
                    operator: Operator::Add,
                    a: Box::new(Node::Number(9.)),
                    b: Box::new(Node::Number(5.)),
                }),
            }
        );

        let tokens = [
            Token {
                token: TokenType::Number,
                value: "6".to_string(),
                start: (0, 0),
                end: (0, 0),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 1),
                end: (0, 1),
            },
            Token {
                token: TokenType::OpeningParenthesis,
                value: "(".to_string(),
                start: (0, 2),
                end: (0, 2),
            },
            Token {
                token: TokenType::Number,
                value: "9".to_string(),
                start: (0, 3),
                end: (0, 3),
            },
            Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: (0, 4),
                end: (0, 4),
            },
            Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: (0, 5),
                end: (0, 5),
            },
            Token {
                token: TokenType::ClosingParenthesis,
                value: ")".to_string(),
                start: (0, 6),
                end: (0, 6),
            },
            Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: (0, 7),
                end: (0, 7),
            },
            Token {
                token: TokenType::Number,
                value: "7".to_string(),
                start: (0, 8),
                end: (0, 8),
            },
        ];
        assert_eq!(
            parse_parenthesis(&mut tokens.iter().peekable(), (&(0, 2), &(0, 6))).unwrap(),
            Node::Operation {
                operator: Operator::Multiply,
                a: Box::new(Node::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Node::Number(6.)),
                    b: Box::new(Node::Operation {
                        operator: Operator::Add,
                        a: Box::new(Node::Number(9.)),
                        b: Box::new(Node::Number(5.)),
                    }),
                }),
                b: Box::new(Node::Number(7.)),
            }
        );
    }
}
