use core::panic;
use std::iter::Peekable;

use nilang_lexer::tokens::{Token, TokenType};
use nodes::{Node, Operator};

pub mod nodes;

const UNEXPECTED_ERROR: &str = "This does not happen, what the fuck are you doing?";
const UNEXPECTED_END_OF_INPUT_ERROR: &str = "Unexpected end of input!";

pub fn parse(tokens: &[Token]) -> Vec<Node> {
    let mut tokens = tokens.iter().peekable();

    let mut program = Vec::new();
    while let Some(_) = tokens.peek() {
        let node = convert(&mut program, &mut tokens);
        program.push(node);
    }

    program
}

fn convert<'a, I>(program: &mut Vec<Node>, tokens: &mut Peekable<I>) -> Node
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
            TokenType::Operator => convert_operation(program, tokens, tkn),
            TokenType::OpeningParenthesis => convert_parenthesis(tokens, (start, end)),
            TokenType::ClosingParenthesis => panic!("[{}] Unexpected closing parenthesis", start),
            TokenType::OpeningBrace => convert_scope(tokens),
            TokenType::ClosingBrace => panic!("[{}] Unexpected closing brace", start),
            TokenType::Keyword => match value.as_str() {
                "rt" => Node::Return(Box::new(convert(program, tokens))),
                "fn" => convert_function(tokens, tkn),
                _ => panic!("{}", UNEXPECTED_ERROR),
            },
            TokenType::Literal => panic!("{}", UNEXPECTED_ERROR),
        };
    } else {
        panic!("{}", UNEXPECTED_END_OF_INPUT_ERROR);
    }
}

fn convert_function<'a, I>(
    tokens: &mut Peekable<I>,
    Token {
        token: _,
        value: _,
        start: _,
        end,
    }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    Node::Function {
        name: match tokens.next() {
            Some(Token {
                token: TokenType::Literal,
                value,
                ..
            }) => value.to_owned(),
            _ => panic!("[{}] Expected a function name", end + 1),
        },
        parameters: if let (
            Some(Token {
                token: TokenType::OpeningParenthesis,
                ..
            }),
            Some(Token {
                token: TokenType::ClosingParenthesis,
                ..
            }),
        ) = (tokens.next(), tokens.next())
        {
            Vec::new()
        } else {
            todo!()
        },
        body: Box::new({
            if let scope @ Node::Scope(_) = convert(&mut Vec::new(), tokens) {
                scope
            } else {
                panic!("[{}] Expected a scope", end + 1)
            }
        }),
    }
}

fn convert_operation<'a, I>(
    program: &mut Vec<Node>,
    tokens: &mut Peekable<I>,
    Token {
        token: _,
        value,
        start,
        end: _,
    }: &Token,
) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    let operator = match value.as_str() {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "%" => Operator::Modulo,
        _ => panic!("{}", UNEXPECTED_ERROR),
    };

    match program
        .pop()
        .expect(&format!("[{}] Expected a number or an operator", start - 1))
    {
        a @ Node::Number(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(convert(program, tokens)),
        },
        a @ Node::Operation { .. } => extend_operation(a, operator, convert(program, tokens)),
        Node::Return(value) => Node::Return(Box::new(match *value {
            a @ Node::Number(_) => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(convert(program, tokens)),
            },
            a @ Node::Operation { .. } => extend_operation(a, operator, convert(program, tokens)),
            a @ Node::Scope(_) => Node::Operation {
                operator,
                a: Box::new(a),
                b: Box::new(convert(program, tokens)),
            },
            Node::Return(_) | Node::Function { .. } => {
                panic!("{}", UNEXPECTED_ERROR)
            }
        })),
        a @ Node::Scope(_) => Node::Operation {
            operator,
            a: Box::new(a),
            b: Box::new(convert(program, tokens)),
        },
        Node::Function { .. } => {
            panic!("[{}] Unexpected token", start - 1)
        }
    }
}

fn convert_parenthesis<'a, I>(tokens: &mut Peekable<I>, (start, end): (&usize, &usize)) -> Node
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
            let node = convert(&mut in_parenthesis, tokens);
            in_parenthesis.push(node);
        }
    }

    if in_parenthesis.is_empty() {
        panic!("[{}-{}] Empty parenthesis", start, last_node_end)
    }
    if in_parenthesis.len() > 1 {
        panic!(
            "[{}-{}] Invalid operation in parenthesis",
            start, last_node_end
        )
    }

    in_parenthesis.first().expect(UNEXPECTED_ERROR).to_owned()
}

fn convert_scope<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = &'a Token>,
{
    let mut in_scope = Vec::new();

    while let Some(token) = tokens.peek() {
        if token.token == TokenType::ClosingBrace {
            tokens.next();
            break;
        } else {
            let node = convert(&mut in_scope, tokens);
            in_scope.push(node);
        }
    }

    Node::Scope(in_scope)
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
