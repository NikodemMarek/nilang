use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Operator,
    tokens::{Token, TokenType},
};

type Loc = (usize, usize);

pub trait AssumingIterator: Iterator {
    fn assume_next(&mut self) -> Result<Token, ParserErrors>;

    fn assume_literal(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors>;
    fn assume_identifier(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors>;
    fn assume_keyword(&mut self, keyword: &str) -> Result<(Loc, Loc), ParserErrors>;
    fn assume_operator(&mut self) -> Result<(Loc, Loc, Operator), ParserErrors>;
    fn assume_equals(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_opening_parenthesis(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_closing_parenthesis(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_opening_brace(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_closing_brace(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_comma(&mut self) -> Result<Loc, ParserErrors>;
    fn assume_semicolon(&mut self) -> Result<Loc, ParserErrors>;
}

impl<I: Iterator<Item = Result<Token, LexerErrors>>> AssumingIterator for I {
    #[inline]
    fn assume_next(&mut self) -> Result<Token, ParserErrors> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(ParserErrors::LexerError(e)),
            None => Err(ParserErrors::EndOfInput {
                loc: (usize::MAX, usize::MAX),
            }),
        }
    }

    #[inline]
    fn assume_literal(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Literal(value),
            } => Ok((start, end, value)),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Literal("".into())]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_identifier(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Identifier(value),
            } => Ok((start, end, value)),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Identifier("".into())]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_keyword(&mut self, keyword: &str) -> Result<(Loc, Loc), ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Keyword(value),
            } => {
                if *value != *keyword {
                    Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Keyword(keyword.into())]),
                        loc: (0, 1),
                    })?
                }

                Ok((start, end))
            }
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Keyword(keyword.into())]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_operator(&mut self) -> Result<(Loc, Loc, Operator), ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Operator(operator),
            } => Ok((start, end, operator)),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::OpeningParenthesis]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_equals(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::Equals,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Equals]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_opening_parenthesis(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::OpeningParenthesis,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::OpeningParenthesis]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_closing_parenthesis(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::ClosingParenthesis,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::ClosingParenthesis]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_opening_brace(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::OpeningBrace,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::OpeningBrace]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_closing_brace(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::ClosingBrace,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::ClosingBrace]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_comma(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::Comma,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Comma]),
                loc: start,
            }),
        }
    }

    #[inline]
    fn assume_semicolon(&mut self) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                token: TokenType::Semicolon,
                ..
            } => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Semicolon]),
                loc: start,
            }),
        }
    }
}

pub trait PeekableAssumingIterator: AssumingIterator {
    fn peek_valid(&mut self) -> Result<&Token, ParserErrors>;
}

impl<I: Iterator<Item = Result<Token, LexerErrors>>> PeekableAssumingIterator for Peekable<I> {
    #[inline]
    fn peek_valid(&mut self) -> Result<&Token, ParserErrors> {
        match self.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(ParserErrors::LexerError(e.clone())),
            None => Err(ParserErrors::EndOfInput {
                loc: (usize::MAX, usize::MAX),
            }),
        }
    }
}
