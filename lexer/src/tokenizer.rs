use std::iter::Peekable;

use errors::{LexerErrors, NilangError};
use nilang_types::{
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

pub struct Tokenizer<'a> {
    iter: Peekable<std::str::Chars<'a>>,
    loc: (usize, usize),
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Localizable<TokenType>, NilangError>;

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

                        return Some(Ok(Localizable {
                            location: Location(start.0, start.1, end.0, end.1),
                            object: TokenType::Literal(aggregation.into()),
                        }));
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, end.0, end.1),
                        object: TokenType::Dot,
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
                                    return Some(Ok(Localizable {
                                        location: Location(
                                            start.0, start.1, self.loc.0, self.loc.1,
                                        ),
                                        object: TokenType::Literal(aggregation.into()),
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

                                return Some(Ok(Localizable {
                                    location: Location(start.0, start.1, end.0, end.1),
                                    object: TokenType::Literal(aggregation.into()),
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

                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, end.0, end.1),
                        object: match aggregation.as_str() {
                            "fn" => TokenType::Keyword(Keyword::Function),
                            "vr" => TokenType::Keyword(Keyword::Variable),
                            "rt" => TokenType::Keyword(Keyword::Return),
                            "st" => TokenType::Keyword(Keyword::Structure),
                            _ => TokenType::Identifier(aggregation.into()),
                        },
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
                                    location: Location::at(self.loc.0, self.loc.1),
                                    error: LexerErrors::ExpectedCharacter('"').into(),
                                }));
                            }
                        }
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, end.0, end.1),
                        object: TokenType::Literal(aggregation.into()),
                    }));
                }
                '\'' => {
                    let start = self.loc;

                    let mut aggregation = String::from('\'');
                    self.iter.next();

                    if let Some(c) = self.iter.next() {
                        if c == '\'' {
                            return Some(Err(NilangError {
                                location: Location::at(self.loc.0, self.loc.1),
                                error: LexerErrors::UnexpectedCharacter('\'').into(),
                            }));
                        } else {
                            self.loc.1 += 1;
                            aggregation.push(c);
                        }
                    } else {
                        return Some(Err(NilangError {
                            location: Location::at(self.loc.0, self.loc.1),
                            error: LexerErrors::UnexpectedEndOfFile.into(),
                        }));
                    }

                    if let Some('\'') = self.iter.next() {
                        self.loc.1 += 1;
                        aggregation.push('\'');
                    } else {
                        return Some(Err(NilangError {
                            location: Location::at(self.loc.0, self.loc.1),
                            error: LexerErrors::ExpectedCharacter('\'').into(),
                        }));
                    }

                    let end = self.loc;
                    self.loc.1 += 1;

                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, end.0, end.1),
                        object: TokenType::Literal(aggregation.into()),
                    }));
                }
                '+' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Operator(nilang_types::nodes::Operator::Add),
                    }));
                }
                '-' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Operator(nilang_types::nodes::Operator::Subtract),
                    }));
                }
                '*' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Operator(nilang_types::nodes::Operator::Multiply),
                    }));
                }
                '/' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Operator(nilang_types::nodes::Operator::Divide),
                    }));
                }
                '%' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Operator(nilang_types::nodes::Operator::Modulo),
                    }));
                }
                '(' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::OpeningParenthesis,
                    }));
                }
                ')' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::ClosingParenthesis,
                    }));
                }
                '{' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::OpeningBrace,
                    }));
                }
                '}' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::ClosingBrace,
                    }));
                }
                '=' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Equals,
                    }));
                }
                ';' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Semicolon,
                    }));
                }
                ':' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Colon,
                    }));
                }
                ',' => {
                    let start = self.loc;
                    self.loc.1 += 1;
                    self.iter.next();
                    return Some(Ok(Localizable {
                        location: Location(start.0, start.1, start.0, start.1),
                        object: TokenType::Comma,
                    }));
                }
                char => {
                    return Some(Err(NilangError {
                        location: Location::at(self.loc.0, self.loc.1),
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
        nodes::Operator,
        tokens::{Keyword, TokenType},
    };

    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenizer() {
        let mut iter = Tokenizer::new("5 + 4");

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("5".into())
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Operator(Operator::Add),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("4".into()),
        );

        let mut iter = Tokenizer::new("test(123)");

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Identifier("test".into()),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::OpeningParenthesis,
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("123".into()),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::ClosingParenthesis,
        );

        let mut iter = Tokenizer::new("fn test(abc) {\n    rt abc + 5;\n}");

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Keyword(Keyword::Function),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Identifier("test".into()),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::OpeningParenthesis,
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Identifier("abc".into()),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::ClosingParenthesis,
        );

        assert_eq!(*iter.next().unwrap().unwrap(), TokenType::OpeningBrace);

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Keyword(Keyword::Return),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Identifier("abc".into()),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Operator(Operator::Add),
        );

        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("5".into()),
        );

        assert_eq!(*iter.next().unwrap().unwrap(), TokenType::Semicolon);

        assert_eq!(*iter.next().unwrap().unwrap(), TokenType::ClosingBrace);
    }

    #[test]
    fn test_tokenizer_literals() {
        let mut iter = Tokenizer::new("5");
        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("5".into()),
        );

        let mut iter = Tokenizer::new("5.5");
        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("5.5".into()),
        );

        let mut iter = Tokenizer::new("'t'");
        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("'t'".into()),
        );

        let mut iter = Tokenizer::new("\"test\"");
        assert_eq!(
            *iter.next().unwrap().unwrap(),
            TokenType::Literal("\"test\"".into()),
        );
    }
}
