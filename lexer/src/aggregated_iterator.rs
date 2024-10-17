use std::iter::Peekable;

use errors::LexerErrors;
use nilang_types::tokens::{Token, TokenType};

pub struct AggregatedIterator<'a> {
    iter: Peekable<std::str::Chars<'a>>,
    loc: (usize, usize),
}

impl<'a> Iterator for AggregatedIterator<'a> {
    type Item = Result<Token, LexerErrors>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(char) = self.iter.peek() {
            match char {
                '\n' => {
                    self.loc.0 += 1;
                    self.loc.1 = 0;

                    self.iter.next();
                }
                ' ' => {
                    self.loc.1 += 1;

                    self.iter.next();
                }
                '\t' => {
                    self.loc.1 += 4; // TOFIX: This is not accurate

                    self.iter.next();
                }
                '.' => {
                    let start = self.loc;

                    let mut aggregation = String::from(self.iter.next().unwrap());

                    loop {
                        match self.iter.peek() {
                            Some('0'..='9') => {
                                self.loc.1 += 1;

                                aggregation.push(self.iter.next().unwrap());
                            }
                            _ => {
                                let end = self.loc;

                                self.loc.1 += 1;

                                return Some(Ok(Token {
                                    token: TokenType::Literal, // TODO: Add dot token
                                    value: aggregation,
                                    start,
                                    end,
                                }));
                            }
                        }
                    }
                }
                '0'..='9' => {
                    let start = self.loc;

                    let mut aggregation = String::from(self.iter.next().unwrap());
                    let mut dot = false;

                    loop {
                        match self.iter.peek() {
                            Some('0'..='9') => {
                                self.loc.1 += 1;

                                aggregation.push(self.iter.next().unwrap());
                            }
                            Some('.') => {
                                if dot {
                                    return Some(Ok(Token {
                                        token: TokenType::Literal,
                                        value: aggregation,
                                        start,
                                        end: self.loc,
                                    }));
                                } else {
                                    self.loc.1 += 1;

                                    aggregation.push(self.iter.next().unwrap());
                                    dot = true;
                                }
                            }
                            _ => {
                                let end = self.loc;

                                self.loc.1 += 1;

                                return Some(Ok(Token {
                                    token: TokenType::Literal,
                                    value: aggregation,
                                    start,
                                    end,
                                }));
                            }
                        }
                    }
                }
                '_' | 'a'..='z' | 'A'..='Z' => {
                    let start = self.loc;

                    let mut aggregation = String::from(self.iter.next().unwrap());

                    while let Some('_' | 'a'..='z' | 'A'..='Z' | '0'..='9') = self.iter.peek() {
                        self.loc.1 += 1;

                        aggregation.push(self.iter.next().unwrap());
                    }

                    let end = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: match aggregation.as_str() {
                            "fn" | "vr" | "rt" => TokenType::Keyword,
                            _ => TokenType::Identifier,
                        },
                        value: aggregation,
                        start,
                        end,
                    }));
                }
                '+' | '-' | '*' | '/' | '%' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Operator,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                '(' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                ')' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                '{' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::OpeningBrace,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                '}' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::ClosingBrace,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                '=' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Equals,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                ';' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Semicolon,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                ',' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Comma,
                        value: self.iter.next().unwrap().to_string(),
                        start,
                        end: start,
                    }));
                }
                char => {
                    return Some(Err(LexerErrors::UnexpectedCharacter {
                        char: *char,
                        loc: self.loc,
                    }));
                }
            };
        }

        None
    }
}

impl AggregatedIterator<'_> {
    #[inline]
    pub fn new(iter: &str) -> AggregatedIterator {
        AggregatedIterator {
            iter: iter.chars().peekable(),
            loc: (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::tokens::{Token, TokenType};

    use crate::aggregated_iterator::AggregatedIterator;

    #[test]
    fn test_aggregated_iterator() {
        let mut iter = AggregatedIterator::new("5 + 4");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal,
                value: String::from("5"),
                start: (0, 0),
                end: (0, 0),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Operator,
                value: String::from("+"),
                start: (0, 2),
                end: (0, 2),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal,
                value: String::from("4"),
                start: (0, 4),
                end: (0, 4),
            }
        );

        let mut iter = AggregatedIterator::new("test(123)");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier,
                value: String::from("test"),
                start: (0, 0),
                end: (0, 3),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningParenthesis,
                value: String::from("("),
                start: (0, 4),
                end: (0, 4),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal,
                value: String::from("123"),
                start: (0, 5),
                end: (0, 7),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingParenthesis,
                value: String::from(")"),
                start: (0, 8),
                end: (0, 8),
            }
        );

        let mut iter = AggregatedIterator::new("fn test(abc) {\n    rt abc + 5;\n}");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Keyword,
                value: String::from("fn"),
                start: (0, 0),
                end: (0, 1),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier,
                value: String::from("test"),
                start: (0, 3),
                end: (0, 6),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningParenthesis,
                value: String::from("("),
                start: (0, 7),
                end: (0, 7),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier,
                value: String::from("abc"),
                start: (0, 8),
                end: (0, 10),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingParenthesis,
                value: String::from(")"),
                start: (0, 11),
                end: (0, 11),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningBrace,
                value: String::from("{"),
                start: (0, 13),
                end: (0, 13),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Keyword,
                value: String::from("rt"),
                start: (1, 4),
                end: (1, 5),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier,
                value: String::from("abc"),
                start: (1, 7),
                end: (1, 9),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Operator,
                value: String::from("+"),
                start: (1, 11),
                end: (1, 11),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal,
                value: String::from("5"),
                start: (1, 13),
                end: (1, 13),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Semicolon,
                value: String::from(";"),
                start: (1, 14),
                end: (1, 14),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingBrace,
                value: String::from("}"),
                start: (2, 0),
                end: (2, 0),
            }
        );
    }
}
