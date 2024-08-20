use tokens::{Token, TokenType};

pub mod tokens;

pub fn lex(input: &str) -> Vec<tokens::Token> {
    let mut chars = input.chars().enumerate().peekable();

    let mut tokens = Vec::new();

    while let Some((index, char)) = chars.next() {
        match &char {
            '0'..='9' | '.' => {
                let mut collector: Token = Token {
                    token: TokenType::Number,
                    value: String::from(char),
                    start: index,
                    end: index,
                };

                while let Some((_, char)) = chars.peek() {
                    match char {
                        '0'..='9' | '.' => {
                            let (index, char) = chars.next().unwrap();
                            collector.end = index;
                            collector.value.push(char);
                        }
                        _ => break,
                    }
                }

                tokens.push(collector);
            }
            '+' | '-' | '*' | '/' | '%' => {
                tokens.push(Token {
                    token: TokenType::Operator,
                    value: char.to_string(),
                    start: index,
                    end: index,
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut collector: Token = Token {
                    token: TokenType::Number,
                    value: String::from(char),
                    start: index,
                    end: index,
                };
                while let Some((_, next)) = chars.peek() {
                    match next {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            let (index, next) = chars.next().unwrap();
                            collector.end = index;
                            collector.value.push(next);
                        }
                        _ => break,
                    }
                }

                tokens.push(Token {
                    token: match collector.value.as_str() {
                        "fn" | "rt" => TokenType::Keyword,
                        _ => TokenType::Literal,
                    },
                    ..collector
                });
            }
            '(' => {
                tokens.push(Token {
                    token: TokenType::OpeningParenthesis,
                    value: char.to_string(),
                    start: index,
                    end: index,
                });
            }
            ')' => {
                tokens.push(Token {
                    token: TokenType::ClosingParenthesis,
                    value: char.to_string(),
                    start: index,
                    end: index,
                });
            }
            '{' => {
                tokens.push(Token {
                    token: TokenType::OpeningBrace,
                    value: char.to_string(),
                    start: index,
                    end: index,
                });
            }
            '}' => {
                tokens.push(Token {
                    token: TokenType::ClosingBrace,
                    value: char.to_string(),
                    start: index,
                    end: index,
                });
            }
            ' ' | '\n' | '\t' => {}
            _ => panic!("Unexpected character: {}", char),
        }
    }

    tokens
}

#[cfg(test)]
mod tests;
