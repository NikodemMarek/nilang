use std::iter::Peekable;

use token::{Program, Token};

pub mod token;

pub fn parse(input: &str) -> Program {
    let mut program = Program {
        program: Vec::new(),
    };

    let mut chars = input.chars().peekable();

    while let Some(pointer) = chars.peek() {
        match pointer {
            '0'..='9' | '.' => program
                .program
                .push(Token::Number(collect_number(&mut chars))),
            '+' | '-' | '*' | '/' | '%' => {
                let operator = match chars.next().unwrap() {
                    '+' => token::Operator::Add,
                    '-' => token::Operator::Subtract,
                    '*' => token::Operator::Multiply,
                    '/' => token::Operator::Divide,
                    '%' => token::Operator::Modulo,
                    _ => panic!("This does not happen, what the fuck are you doing?"),
                };

                let a = match program
                    .program
                    .pop()
                    .expect("Expected a value before operator")
                {
                    a @ Token::Number(_) => a,
                    _ => panic!("Expected a number before operator"),
                };
                let a = Box::new(a);

                discard_spaces(&mut chars);
                let b = Box::new(Token::Number(collect_number(&mut chars)));

                program.program.push(Token::Operator { operator, a, b });
            }
            ' ' => {
                chars.next();
            }
            _ => panic!("Invalid character in position: {pointer}"),
        }
    }

    program
}

fn collect_number(chars: &mut Peekable<std::str::Chars>) -> f64 {
    let mut number = String::new();

    while let Some(pointer) = &chars.peek() {
        match pointer {
            '0'..='9' | '.' => {
                number.push(**pointer);
                chars.next();
            }
            ' ' | '+' | '-' | '*' | '/' | '%' => break,
            _ => panic!("Invalid character in position: {pointer}"),
        }
    }

    number.parse().expect("Failed to parse number")
}

fn discard_spaces(chars: &mut Peekable<std::str::Chars>) {
    while let Some(pointer) = chars.peek() {
        if pointer.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse,
        token::{Operator, Token},
    };

    #[test]
    fn parse_numbers() {
        assert_eq!(parse("54").program, vec![Token::Number(54.)]);
        assert_eq!(parse("8.5").program, vec![Token::Number(8.5)]);
        assert_eq!(parse(".2").program, vec![Token::Number(0.2)]);
        assert_eq!(parse("6.").program, vec![Token::Number(6.)]);
    }

    #[test]
    fn parse_simple_operations() {
        assert_eq!(
            parse("6 + 9").program,
            vec![Token::Operator {
                operator: Operator::Add,
                a: Box::new(Token::Number(6.)),
                b: Box::new(Token::Number(9.)),
            },]
        );

        assert_eq!(
            parse("5 - 7.5").program,
            vec![Token::Operator {
                operator: Operator::Subtract,
                a: Box::new(Token::Number(5.)),
                b: Box::new(Token::Number(7.5)),
            }]
        );

        assert_eq!(
            parse(".3 * 4").program,
            vec![Token::Operator {
                operator: Operator::Multiply,
                a: Box::new(Token::Number(0.3)),
                b: Box::new(Token::Number(4.)),
            }]
        );

        assert_eq!(
            parse("2. / 1").program,
            vec![Token::Operator {
                operator: Operator::Divide,
                a: Box::new(Token::Number(2.)),
                b: Box::new(Token::Number(1.)),
            }]
        );
    }
}
