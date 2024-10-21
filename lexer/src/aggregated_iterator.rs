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
                                    token: TokenType::Literal(aggregation.into()), // TODO: Add dot token
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
                                        token: TokenType::Literal(aggregation.into()),
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
                                    token: TokenType::Literal(aggregation.into()),
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
                            "fn" | "vr" | "rt" => TokenType::Keyword(aggregation.into()),
                            _ => TokenType::Identifier(aggregation.into()),
                        },
                        start,
                        end,
                    }));
                }
                '+' | '-' | '*' | '/' | '%' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    let value = self.iter.next().unwrap();
                    return Some(Ok(Token {
                        token: TokenType::Operator(match value {
                            '+' => nilang_types::nodes::Operator::Add,
                            '-' => nilang_types::nodes::Operator::Subtract,
                            '*' => nilang_types::nodes::Operator::Multiply,
                            '/' => nilang_types::nodes::Operator::Divide,
                            '%' => nilang_types::nodes::Operator::Modulo,
                            _ => unreachable!(),
                        }),
                        start,
                        end: start,
                    }));
                }
                '(' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::OpeningParenthesis,
                        start,
                        end: start,
                    }));
                }
                ')' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::ClosingParenthesis,
                        start,
                        end: start,
                    }));
                }
                '{' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::OpeningBrace,
                        start,
                        end: start,
                    }));
                }
                '}' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::ClosingBrace,
                        start,
                        end: start,
                    }));
                }
                '=' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Equals,
                        start,
                        end: start,
                    }));
                }
                ';' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Semicolon,
                        start,
                        end: start,
                    }));
                }
                ',' => {
                    let start = self.loc;

                    self.loc.1 += 1;

                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Comma,
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
    pub fn new(iter: &str) -> AggregatedIterator<'_> {
        AggregatedIterator {
            iter: iter.chars().peekable(),
            loc: (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Operator,
        tokens::{Token, TokenType},
    };

    use crate::aggregated_iterator::AggregatedIterator;

    #[test]
    fn test_aggregated_iterator() {
        let mut iter = AggregatedIterator::new("5 + 4");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("5".into()),
                start: (0, 0),
                end: (0, 0),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Operator(Operator::Add),
                start: (0, 2),
                end: (0, 2),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("4".into()),
                start: (0, 4),
                end: (0, 4),
            }
        );

        let mut iter = AggregatedIterator::new("test(123)");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier("test".into()),
                start: (0, 0),
                end: (0, 3),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningParenthesis,
                start: (0, 4),
                end: (0, 4),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("123".into()),
                start: (0, 5),
                end: (0, 7),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingParenthesis,
                start: (0, 8),
                end: (0, 8),
            }
        );

        let mut iter = AggregatedIterator::new("fn test(abc) {\n    rt abc + 5;\n}");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Keyword("fn".into()),
                start: (0, 0),
                end: (0, 1),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier("test".into()),
                start: (0, 3),
                end: (0, 6),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningParenthesis,
                start: (0, 7),
                end: (0, 7),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier("abc".into()),
                start: (0, 8),
                end: (0, 10),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingParenthesis,
                start: (0, 11),
                end: (0, 11),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::OpeningBrace,
                start: (0, 13),
                end: (0, 13),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Keyword("rt".into()),
                start: (1, 4),
                end: (1, 5),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Identifier("abc".into()),
                start: (1, 7),
                end: (1, 9),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Operator(Operator::Add),
                start: (1, 11),
                end: (1, 11),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("5".into()),
                start: (1, 13),
                end: (1, 13),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Semicolon,
                start: (1, 14),
                end: (1, 14),
            }
        );

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::ClosingBrace,
                start: (2, 0),
                end: (2, 0),
            }
        );
    }
}
