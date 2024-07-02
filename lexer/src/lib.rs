use tokens::{Token, TokenType};

mod tokens;

pub fn convert(input: &str) -> Vec<tokens::Token> {
    let mut chars = input.chars().peekable().enumerate();

    let mut tokens = Vec::new();
    let mut collector: Option<Token> = None;

    while let Some((index, pointer)) = chars.next() {
        match &pointer {
            '0'..='9' | '.' => {
                if let Some(token) = collector.as_mut() {
                    if token.token == TokenType::Number {
                        token.end = index;
                        token.value.push(pointer);
                        continue;
                    } else {
                        tokens.push(token.clone());
                    }
                }

                collector = Some(Token {
                    token: TokenType::Number,
                    value: pointer.to_string(),
                    start: index,
                    end: index,
                });
            }
            '+' | '-' | '*' | '/' | '%' => {
                if let Some(token) = collector.as_mut() {
                    if token.token == TokenType::Operator {
                        token.end = index;
                        token.value.push(pointer);
                        continue;
                    } else {
                        tokens.push(token.clone());
                    }
                }

                collector = Some(Token {
                    token: TokenType::Operator,
                    value: pointer.to_string(),
                    start: index,
                    end: index,
                });
            }
            ' ' => {
                if let Some(token) = collector.as_ref() {
                    tokens.push(token.clone());
                    collector = None;
                }
            }
            _ => panic!("Unexpected character: {}", pointer),
        }
    }

    if let Some(token) = collector {
        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_numbers() {
        assert_eq!(
            convert("5  "),
            vec![Token {
                token: TokenType::Number,
                value: "5".to_string(),
                start: 0,
                end: 0,
            }]
        );
        assert_eq!(
            convert("4.  "),
            vec![Token {
                token: TokenType::Number,
                value: "4.".to_string(),
                start: 0,
                end: 1,
            }]
        );
        assert_eq!(
            convert(".9"),
            vec![Token {
                token: TokenType::Number,
                value: ".9".to_string(),
                start: 0,
                end: 1,
            }]
        );
        assert_eq!(
            convert("3.7"),
            vec![Token {
                token: TokenType::Number,
                value: "3.7".to_string(),
                start: 0,
                end: 2,
            }]
        );
    }

    #[test]
    fn convert_operators() {
        assert_eq!(
            convert("  +"),
            vec![Token {
                token: TokenType::Operator,
                value: "+".to_string(),
                start: 2,
                end: 2,
            }]
        );
        assert_eq!(
            convert(" - "),
            vec![Token {
                token: TokenType::Operator,
                value: "-".to_string(),
                start: 1,
                end: 1,
            }]
        );
        assert_eq!(
            convert("*"),
            vec![Token {
                token: TokenType::Operator,
                value: "*".to_string(),
                start: 0,
                end: 0,
            }]
        );
        assert_eq!(
            convert("/"),
            vec![Token {
                token: TokenType::Operator,
                value: "/".to_string(),
                start: 0,
                end: 0,
            }]
        );
        assert_eq!(
            convert("%"),
            vec![Token {
                token: TokenType::Operator,
                value: "%".to_string(),
                start: 0,
                end: 0,
            }]
        );
    }

    #[test]
    fn convert_operations() {
        assert_eq!(
            convert("5+4"),
            vec![
                Token {
                    token: TokenType::Number,
                    value: "5".to_string(),
                    start: 0,
                    end: 0,
                },
                Token {
                    token: TokenType::Operator,
                    value: "+".to_string(),
                    start: 1,
                    end: 1,
                },
                Token {
                    token: TokenType::Number,
                    value: "4".to_string(),
                    start: 2,
                    end: 2,
                },
            ]
        );
        assert_eq!(
            convert("5.5 * 8"),
            vec![
                Token {
                    token: TokenType::Number,
                    value: "5.5".to_string(),
                    start: 0,
                    end: 2,
                },
                Token {
                    token: TokenType::Operator,
                    value: "*".to_string(),
                    start: 4,
                    end: 4,
                },
                Token {
                    token: TokenType::Number,
                    value: "8".to_string(),
                    start: 6,
                    end: 6,
                },
            ]
        );
    }
}
