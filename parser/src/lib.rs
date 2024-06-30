use core::panic;
use std::iter::Peekable;

use token::{Operator, Program, Token};

pub mod token;

pub fn parse(input: &str) -> Program {
    let mut program = Program {
        program: Vec::new(),
    };

    let mut chars = input.chars().peekable();

    while let Some(pointer) = chars.peek() {
        match pointer {
            '0'..='9' | '.' => program.program.push(collect_number(&mut chars)),
            '+' | '-' | '*' | '/' | '%' => {
                let operator = match chars.next().unwrap() {
                    '+' => token::Operator::Add,
                    '-' => token::Operator::Subtract,
                    '*' => token::Operator::Multiply,
                    '/' => token::Operator::Divide,
                    '%' => token::Operator::Modulo,
                    _ => panic!("This does not happen, what the fuck are you doing?"),
                };

                let token = match program
                    .program
                    .pop()
                    .expect("Expected a value before operator")
                {
                    a @ Token::Number(_) => {
                        let a = Box::new(a);

                        discard_spaces(&mut chars);
                        let b = Box::new(collect_number(&mut chars));

                        Token::Operation { operator, a, b }
                    }
                    Token::Operation {
                        operator: prev_operator,
                        a: prev_a,
                        b: prev_b,
                    } => match operator {
                        Operator::Add | Operator::Subtract => match prev_operator {
                            Operator::Add | Operator::Subtract => Token::Operation {
                                operator,
                                a: Box::new(Token::Operation {
                                    operator: prev_operator,
                                    a: prev_a,
                                    b: prev_b,
                                }),
                                b: Box::new({
                                    discard_spaces(&mut chars);
                                    collect_number(&mut chars)
                                }),
                            },
                            Operator::Multiply | Operator::Divide | Operator::Modulo => {
                                Token::Operation {
                                    operator,
                                    a: Box::new(Token::Operation {
                                        operator: prev_operator,
                                        a: prev_a,
                                        b: prev_b,
                                    }),
                                    b: Box::new({
                                        discard_spaces(&mut chars);
                                        collect_number(&mut chars)
                                    }),
                                }
                            }
                        },
                        Operator::Multiply | Operator::Divide | Operator::Modulo => {
                            match prev_operator {
                                Operator::Add | Operator::Subtract => Token::Operation {
                                    operator: prev_operator,
                                    a: prev_a,
                                    b: Box::new(Token::Operation {
                                        operator,
                                        a: prev_b,
                                        b: Box::new({
                                            discard_spaces(&mut chars);
                                            collect_number(&mut chars)
                                        }),
                                    }),
                                },
                                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                                    Token::Operation {
                                        operator,
                                        a: Box::new(Token::Operation {
                                            operator: prev_operator,
                                            a: prev_a,
                                            b: prev_b,
                                        }),
                                        b: Box::new({
                                            discard_spaces(&mut chars);
                                            collect_number(&mut chars)
                                        }),
                                    }
                                }
                            }
                        }
                    },
                };

                program.program.push(token);
            }
            ' ' => {
                chars.next();
            }
            _ => panic!("Invalid character in position: {pointer}"),
        }
    }

    program
}

fn collect_number(chars: &mut Peekable<std::str::Chars>) -> Token {
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

    Token::Number(number.parse().expect("Failed to parse number"))
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
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Number(6.)),
                b: Box::new(Token::Number(9.)),
            }]
        );

        assert_eq!(
            parse("5 - 7.5").program,
            vec![Token::Operation {
                operator: Operator::Subtract,
                a: Box::new(Token::Number(5.)),
                b: Box::new(Token::Number(7.5)),
            }]
        );

        assert_eq!(
            parse(".3 * 4").program,
            vec![Token::Operation {
                operator: Operator::Multiply,
                a: Box::new(Token::Number(0.3)),
                b: Box::new(Token::Number(4.)),
            }]
        );

        assert_eq!(
            parse("2. / 1").program,
            vec![Token::Operation {
                operator: Operator::Divide,
                a: Box::new(Token::Number(2.)),
                b: Box::new(Token::Number(1.)),
            }]
        );

        assert_eq!(
            parse("5 % 1.5").program,
            vec![Token::Operation {
                operator: Operator::Modulo,
                a: Box::new(Token::Number(5.)),
                b: Box::new(Token::Number(1.5)),
            }]
        );
    }

    #[test]
    fn parse_complex_operations() {
        assert_eq!(
            parse("6 + 9 + 5").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Operation {
                    operator: Operator::Add,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(9.)),
                }),
                b: Box::new(Token::Number(5.)),
            }]
        );
        assert_eq!(
            parse("6 - 9 + 5").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Operation {
                    operator: Operator::Subtract,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(9.)),
                }),
                b: Box::new(Token::Number(5.)),
            }]
        );
        assert_eq!(
            parse("6 + 9 - 5").program,
            vec![Token::Operation {
                operator: Operator::Subtract,
                a: Box::new(Token::Operation {
                    operator: Operator::Add,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(9.)),
                }),
                b: Box::new(Token::Number(5.)),
            }]
        );

        assert_eq!(
            parse("6 * .5 * 7").program,
            vec![Token::Operation {
                operator: Operator::Multiply,
                a: Box::new(Token::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(0.5)),
                }),
                b: Box::new(Token::Number(7.)),
            }]
        );
        assert_eq!(
            parse("6 % 5 * .7").program,
            vec![Token::Operation {
                operator: Operator::Multiply,
                a: Box::new(Token::Operation {
                    operator: Operator::Modulo,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(5.)),
                }),
                b: Box::new(Token::Number(0.7)),
            }]
        );
        assert_eq!(
            parse("6 * .5 / 7").program,
            vec![Token::Operation {
                operator: Operator::Divide,
                a: Box::new(Token::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Token::Number(6.)),
                    b: Box::new(Token::Number(0.5)),
                }),
                b: Box::new(Token::Number(7.)),
            }]
        );

        assert_eq!(
            parse("4 + 5 * 3").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Number(4.)),
                b: Box::new(Token::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Token::Number(5.)),
                    b: Box::new(Token::Number(3.)),
                }),
            },]
        );
        assert_eq!(
            parse(".2 * 5.5 + 8").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Token::Number(0.2)),
                    b: Box::new(Token::Number(5.5)),
                }),
                b: Box::new(Token::Number(8.)),
            }]
        );

        assert_eq!(
            parse("2 % 5 + 8.5 * .7").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Operation {
                    operator: Operator::Modulo,
                    a: Box::new(Token::Number(2.)),
                    b: Box::new(Token::Number(5.)),
                }),
                b: Box::new(Token::Operation {
                    operator: Operator::Multiply,
                    a: Box::new(Token::Number(8.5)),
                    b: Box::new(Token::Number(0.7)),
                }),
            }]
        );
        assert_eq!(
            parse(".2 + 5.5 * 8 + .7").program,
            vec![Token::Operation {
                operator: Operator::Add,
                a: Box::new(Token::Operation {
                    operator: Operator::Add,
                    a: Box::new(Token::Number(0.2)),
                    b: Box::new(Token::Operation {
                        operator: Operator::Multiply,
                        a: Box::new(Token::Number(5.5)),
                        b: Box::new(Token::Number(8.)),
                    }),
                }),
                b: Box::new(Token::Number(0.7)),
            }]
        );
    }
}
