use std::iter::Peekable;

use errors::{CodeLocation, LexerErrors, NilangError};
use nilang_types::{
    nodes::expressions::{Arithmetic, Boolean, Operator},
    tokens::{Keyword, Token, TokenType},
};

pub struct Tokenizer<'a> {
    iter: Peekable<std::str::Chars<'a>>,
    loc: (usize, usize),
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, NilangError>;

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

                    let mut aggregation = String::from('.');
                    self.iter.next();

                    if let Some('0'..='9') = self.iter.peek() {
                        while let Some(c @ '0'..='9') = self.iter.peek() {
                            self.loc.1 += 1;

                            aggregation.push(*c);
                            self.iter.next();
                        }

                        let end = self.loc;
                        self.loc.1 += 1;

                        return Some(Ok(Token {
                            token: TokenType::Literal(aggregation.into()),
                            start,
                            end,
                        }));
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Dot,
                        start,
                        end,
                    }));
                }
                c @ '0'..='9' => {
                    let start = self.loc;

                    let mut aggregation = String::from(*c);
                    self.iter.next();

                    let mut dot = false;
                    loop {
                        match self.iter.peek() {
                            Some(c @ '0'..='9') => {
                                self.loc.1 += 1;
                                aggregation.push(*c);
                                self.iter.next();
                            }
                            Some(c @ '.') => {
                                if dot {
                                    return Some(Ok(Token {
                                        token: TokenType::Literal(aggregation.into()),
                                        start,
                                        end: self.loc,
                                    }));
                                } else {
                                    self.loc.1 += 1;
                                    aggregation.push(*c);
                                    self.iter.next();

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
                c @ '_' | c @ 'a'..='z' | c @ 'A'..='Z' => {
                    let start = self.loc;

                    let mut aggregation = String::from(*c);
                    self.iter.next();

                    while let Some(c @ '_' | c @ 'a'..='z' | c @ 'A'..='Z' | c @ '0'..='9') =
                        self.iter.peek()
                    {
                        self.loc.1 += 1;
                        aggregation.push(*c);
                        self.iter.next();
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: match aggregation.as_str() {
                            "fn" => TokenType::Keyword(Keyword::Function),
                            "vr" => TokenType::Keyword(Keyword::Variable),
                            "rt" => TokenType::Keyword(Keyword::Return),
                            "st" => TokenType::Keyword(Keyword::Structure),
                            "if" => TokenType::Keyword(Keyword::If),
                            "ef" => TokenType::Keyword(Keyword::ElseIf),
                            "el" => TokenType::Keyword(Keyword::Else),
                            "wl" => TokenType::Keyword(Keyword::While),
                            "true" | "false" => TokenType::Literal(aggregation.into()),
                            _ => TokenType::Identifier(aggregation.into()),
                        },
                        start,
                        end,
                    }));
                }
                '"' => {
                    let start = self.loc;

                    let mut aggregation = String::from('"');
                    self.iter.next();

                    loop {
                        match self.iter.next() {
                            Some('"') => {
                                self.loc.1 += 1;
                                aggregation.push('"');
                                break;
                            }
                            Some(char) => {
                                self.loc.1 += 1;
                                aggregation.push(char);
                            }
                            None => {
                                return Some(Err(NilangError {
                                    location: CodeLocation::at(self.loc.0, self.loc.1),
                                    error: LexerErrors::ExpectedCharacter('"').into(),
                                }));
                            }
                        }
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Literal(aggregation.into()),
                        start,
                        end,
                    }));
                }
                '\'' => {
                    let start = self.loc;

                    let mut aggregation = String::from('\'');
                    self.iter.next();

                    if let Some(c) = self.iter.next() {
                        if c == '\'' {
                            return Some(Err(NilangError {
                                location: CodeLocation::at(self.loc.0, self.loc.1),
                                error: LexerErrors::UnexpectedCharacter('\'').into(),
                            }));
                        } else {
                            self.loc.1 += 1;
                            aggregation.push(c);
                        }
                    } else {
                        return Some(Err(NilangError {
                            location: CodeLocation::at(self.loc.0, self.loc.1),
                            error: LexerErrors::UnexpectedEndOfFile.into(),
                        }));
                    }

                    if let Some('\'') = self.iter.next() {
                        self.loc.1 += 1;
                        aggregation.push('\'');
                    } else {
                        return Some(Err(NilangError {
                            location: CodeLocation::at(self.loc.0, self.loc.1),
                            error: LexerErrors::ExpectedCharacter('\'').into(),
                        }));
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Token {
                        token: TokenType::Literal(aggregation.into()),
                        start,
                        end,
                    }));
                }
                '+' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Add)),
                        start,
                        end: start,
                    }));
                }
                '-' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Subtract)),
                        start,
                        end: start,
                    }));
                }
                '*' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Multiply)),
                        start,
                        end: start,
                    }));
                }
                '/' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Divide)),
                        start,
                        end: start,
                    }));
                }
                '%' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Modulo)),
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

                    let token = if let Some('=') = self.iter.peek() {
                        self.loc.1 += 1;
                        self.iter.next();

                        TokenType::Operator(Operator::Boolean(Boolean::Equal))
                    } else {
                        TokenType::Equals
                    };

                    return Some(Ok(Token {
                        token,
                        start,
                        end: (self.loc.0, self.loc.1 - 1),
                    }));
                }
                '!' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();

                    let token = if let Some('=') = self.iter.peek() {
                        self.loc.1 += 1;
                        self.iter.next();

                        TokenType::Operator(Operator::Boolean(Boolean::NotEqual))
                    } else {
                        return Some(Err(NilangError {
                            location: CodeLocation::at(self.loc.0, self.loc.1),
                            error: LexerErrors::UnexpectedCharacter('!').into(),
                        }));
                    };

                    return Some(Ok(Token {
                        token,
                        start,
                        end: (self.loc.0, self.loc.1 - 1),
                    }));
                }
                '<' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();

                    let token = if let Some('=') = self.iter.peek() {
                        self.loc.1 += 1;
                        self.iter.next();

                        TokenType::Operator(Operator::Boolean(Boolean::LessOrEqual))
                    } else {
                        TokenType::Operator(Operator::Boolean(Boolean::Less))
                    };

                    return Some(Ok(Token {
                        token,
                        start,
                        end: (self.loc.0, self.loc.1 - 1),
                    }));
                }
                '>' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();

                    let token = if let Some('=') = self.iter.peek() {
                        self.loc.1 += 1;
                        self.iter.next();

                        TokenType::Operator(Operator::Boolean(Boolean::MoreOrEqual))
                    } else {
                        TokenType::Operator(Operator::Boolean(Boolean::More))
                    };

                    return Some(Ok(Token {
                        token,
                        start,
                        end: (self.loc.0, self.loc.1 - 1),
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
                ':' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Token {
                        token: TokenType::Colon,
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
                    return Some(Err(NilangError {
                        location: CodeLocation::at(self.loc.0, self.loc.1),
                        error: LexerErrors::UnexpectedCharacter(*char).into(),
                    }));
                }
            };
        }

        None
    }
}

impl Tokenizer<'_> {
    #[inline]
    pub fn new(iter: &str) -> Tokenizer<'_> {
        Tokenizer {
            iter: iter.chars().peekable(),
            loc: (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::expressions::{Arithmetic, Operator},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenizer() {
        let mut iter = Tokenizer::new("5 + 4");

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
                token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Add)),
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

        let mut iter = Tokenizer::new("test(123)");

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

        let mut iter = Tokenizer::new("fn test(abc) {\n    rt abc + 5;\n}");

        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Keyword(Keyword::Function),
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
                token: TokenType::Keyword(Keyword::Return),
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
                token: TokenType::Operator(Operator::Arithmetic(Arithmetic::Add)),
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

    #[test]
    fn test_tokenizer_literals() {
        let mut iter = Tokenizer::new("true");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("true".into()),
                start: (0, 0),
                end: (0, 3),
            }
        );

        let mut iter = Tokenizer::new("false");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("false".into()),
                start: (0, 0),
                end: (0, 4),
            }
        );

        let mut iter = Tokenizer::new("5");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("5".into()),
                start: (0, 0),
                end: (0, 0),
            }
        );

        let mut iter = Tokenizer::new("5.5");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("5.5".into()),
                start: (0, 0),
                end: (0, 2),
            }
        );

        let mut iter = Tokenizer::new("'t'");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("'t'".into()),
                start: (0, 0),
                end: (0, 2),
            }
        );

        let mut iter = Tokenizer::new("\"test\"");
        assert_eq!(
            iter.next().unwrap().unwrap(),
            Token {
                token: TokenType::Literal("\"test\"".into()),
                start: (0, 0),
                end: (0, 5),
            }
        );
    }
}
