use std::iter::Peekable;

use errors::{LexerErrors, ParserErrors};
use nilang_types::{
    nodes::Operator,
    tokens::{Keyword, Token, TokenType},
};

type Loc = (usize, usize);

pub trait AssumingIterator: Iterator {
    fn assume_next(&mut self) -> Result<Token, ParserErrors>;

    fn assume(&mut self, token: TokenType) -> Result<Loc, ParserErrors>;
    fn assume_literal(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors>;
    fn assume_identifier(&mut self) -> Result<(Loc, Loc, Box<str>), ParserErrors>;
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<(Loc, Loc), ParserErrors>;
    fn assume_operator(&mut self) -> Result<(Loc, Loc, Operator), ParserErrors>;
}

impl<I: Iterator<Item = Result<Token, LexerErrors>>> AssumingIterator for I {
    #[inline]
    fn assume(&mut self, token: TokenType) -> Result<Loc, ParserErrors> {
        match self.assume_next()? {
            Token {
                start, token: t, ..
            } if t == token => Ok(start),
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([token]),
                loc: start,
            }),
        }
    }

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
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<(Loc, Loc), ParserErrors> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Keyword(value),
            } => {
                if value != keyword {
                    Err(ParserErrors::ExpectedTokens {
                        tokens: Vec::from([TokenType::Keyword(keyword)]),
                        loc: (0, 1),
                    })?
                }

                Ok((start, end))
            }
            Token { start, .. } => Err(ParserErrors::ExpectedTokens {
                tokens: Vec::from([TokenType::Keyword(keyword)]),
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
