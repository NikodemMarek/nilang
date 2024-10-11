use eyre::eyre;
use tokens::{Token, TokenType};

pub mod tokens;

pub fn lex(input: &str) -> eyre::Result<Vec<tokens::Token>> {
    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();

    let mut line = 0;
    let mut column = 0;

    while let Some(char) = chars.next() {
        match &char {
            '0'..='9' | '.' => {
                let mut collector: Token = Token {
                    token: TokenType::Number,
                    value: String::from(char),
                    start: (line, column),
                    end: (line, column),
                };

                loop {
                    if let Some(char) = chars.peek() {
                        match char {
                            '0'..='9' | '.' => {
                                column += 1;

                                let char = chars.next().unwrap();
                                collector.end = (line, column);
                                collector.value.push(char);
                            }
                            _ => {
                                tokens.push(collector);

                                column += 1;

                                break;
                            }
                        }
                    } else {
                        tokens.push(collector);

                        return Ok(tokens);
                    }
                }
            }
            '+' | '-' | '*' | '/' | '%' => {
                tokens.push(Token {
                    token: TokenType::Operator,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut collector: Token = Token {
                    token: TokenType::Number,
                    value: String::from(char),
                    start: (line, column),
                    end: (line, column),
                };

                loop {
                    if let Some(next) = chars.peek() {
                        match next {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                column += 1;

                                let next = chars.next().unwrap();
                                collector.end = (line, column);
                                collector.value.push(next);
                            }
                            _ => {
                                tokens.push(Token {
                                    token: match collector.value.as_str() {
                                        "fn" | "rt" | "vr" => TokenType::Keyword,
                                        _ => TokenType::Literal,
                                    },
                                    end: (line, column),
                                    ..collector
                                });

                                column += 1;

                                break;
                            }
                        }
                    } else {
                        tokens.push(Token {
                            token: match collector.value.as_str() {
                                "fn" | "rt" | "vr" => TokenType::Keyword,
                                _ => TokenType::Literal,
                            },
                            end: (line, column),
                            ..collector
                        });

                        return Ok(tokens);
                    }
                }
            }
            '(' => {
                tokens.push(Token {
                    token: TokenType::OpeningParenthesis,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            ')' => {
                tokens.push(Token {
                    token: TokenType::ClosingParenthesis,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            '{' => {
                tokens.push(Token {
                    token: TokenType::OpeningBrace,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            '}' => {
                tokens.push(Token {
                    token: TokenType::ClosingBrace,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            '=' => {
                tokens.push(Token {
                    token: TokenType::Equals,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            ';' => {
                tokens.push(Token {
                    token: TokenType::Semicolon,
                    value: char.to_string(),
                    start: (line, column),
                    end: (line, column),
                });

                column += 1;
            }
            '\n' => {
                line += 1;
                column = 0;
            }
            ' ' => {
                column += 1;
            }
            '\t' => {
                column += 4; // TOFIX: This is not accurate
            }
            _ => Err(eyre!("Invalid character: {}", char))?,
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests;
