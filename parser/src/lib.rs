use core::panic;
use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};
use nodes::{Node, Operator, Program};

pub mod nodes;

const UNEXPECTED_ERROR: &str = "This does not happen, what the fuck are you doing?";
const UNEXPECTED_END_OF_INPUT_ERROR: &str = "Unexpected end of input!";

pub fn parse(tokens: &[Token]) -> Program {
    let mut program = Program {
        program: Vec::new(),
    };

    let mut tokens = tokens.iter().peekable();

    while let Some(_) = tokens.peek() {
        let node = convert(&mut program, &mut tokens);
        program.program.push(node);
    }

    program
}

fn convert<'a, I>(program: &mut Program, tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(
        tkn @ Token {
            token,
            value,
            start,
            end,
        },
    ) = tokens.next()
    {
        return match token {
            TokenType::Number => convert_number(tkn),
            TokenType::Operator => {
                let operator = match &value as &str {
                    "+" => Operator::Add,
                    "-" => Operator::Subtract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    "%" => Operator::Modulo,
                    _ => panic!("{}", UNEXPECTED_ERROR),
                };

                match program
                    .program
                    .pop()
                    .expect(&format!("[{}] Expected a number or an operator", start - 1))
                {
                    a @ Node::Number(_) => Node::Operation {
                        operator,
                        a: Box::new(a),
                        b: Box::new(convert(program, tokens)),
                    },
                    a @ Node::Operation { .. } => {
                        extend_operation(a, operator, convert(program, tokens))
                    }
                    Node::ParenthesisTerminator => {
                        panic!("[{}] Unexpected closing parenthesis", start - 1)
                    }
                }
            }
            TokenType::LeftParenthesis => {
                let mut in_parenthesis = Program {
                    program: Vec::new(),
                };

                let mut last_node_end = end;
                while let Some(Token { end, .. }) = tokens.peek() {
                    last_node_end = end;

                    let node = convert(&mut in_parenthesis, tokens);

                    if let Node::ParenthesisTerminator = node {
                        break;
                    }

                    in_parenthesis.program.push(node);
                }

                if in_parenthesis.program.is_empty() {
                    panic!("[{}-{}] Empty parenthesis", start, last_node_end)
                }
                if in_parenthesis.program.len() > 1 {
                    panic!(
                        "[{}-{}] Invalid operation in parenthesis",
                        start, last_node_end
                    )
                }

                in_parenthesis
                    .program
                    .first()
                    .expect(UNEXPECTED_ERROR)
                    .to_owned()
            }
            TokenType::RightParenthesis => Node::ParenthesisTerminator,
        };
    } else {
        panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
    }
}

fn convert_number(
    Token {
        token,
        value,
        start,
        end,
    }: &Token,
) -> Node {
    if let TokenType::Number = token {
        Node::Number(
            value
                .parse()
                .expect(&format!("[{start}-{end}] Invalid number: \"{value}\"")),
        )
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}

fn extend_operation(operation: Node, operator: Operator, node: Node) -> Node {
    if let Node::Operation {
        operator: prev_operator,
        a: prev_a,
        b: prev_b,
    } = operation
    {
        match operator {
            Operator::Add | Operator::Subtract => match prev_operator {
                Operator::Add | Operator::Subtract => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
            },
            Operator::Multiply | Operator::Divide | Operator::Modulo => match prev_operator {
                Operator::Add | Operator::Subtract => Node::Operation {
                    operator: prev_operator,
                    a: prev_a,
                    b: Box::new(Node::Operation {
                        operator,
                        a: prev_b,
                        b: Box::new(node),
                    }),
                },
                Operator::Multiply | Operator::Divide | Operator::Modulo => Node::Operation {
                    operator,
                    a: Box::new(Node::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    }),
                    b: Box::new(node),
                },
            },
        }
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}

#[cfg(test)]
mod tests;
