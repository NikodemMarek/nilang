use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::Operator,
    tokens::{Keyword, Token, TokenType},
};

use crate::multi_peekable::MultiPeekable;

type Loc = (usize, usize);

pub trait AssumingIterator: Iterator {
    fn assume_next(&mut self) -> Result<Token, NilangError>;

    fn assume(&mut self, token: TokenType) -> Result<Loc, NilangError>;
    fn assume_literal(&mut self) -> Result<(Loc, Loc, Box<str>), NilangError>;
    fn assume_identifier(&mut self) -> Result<(Loc, Loc, Box<str>), NilangError>;
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<(Loc, Loc), NilangError>;
    fn assume_operator(&mut self) -> Result<(Loc, Loc, Operator), NilangError>;
}

impl<I: Iterator<Item = Result<Token, NilangError>>> AssumingIterator for I {
    #[inline]
    fn assume(&mut self, token: TokenType) -> Result<Loc, NilangError> {
        match self.assume_next()? {
            Token {
                start, token: t, ..
            } if t == token => Ok(start),
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([token])).into(),
            }),
        }
    }

    #[inline]
    fn assume_next(&mut self) -> Result<Token, NilangError> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e.clone()),
            None => Err(NilangError {
                location: CodeLocation::at(usize::MAX, usize::MAX),
                error: ParserErrors::EndOfInput.into(),
            }),
        }
    }

    #[inline]
    fn assume_literal(&mut self) -> Result<(Loc, Loc, Box<str>), NilangError> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Literal(value),
            } => Ok((start, end, value)),
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Literal("".into())]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_identifier(&mut self) -> Result<(Loc, Loc, Box<str>), NilangError> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Identifier(value),
            } => Ok((start, end, value)),
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Identifier("".into())]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<(Loc, Loc), NilangError> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Keyword(value),
            } => {
                if value != keyword {
                    Err(NilangError {
                        location: CodeLocation::at(0, 1),
                        error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Keyword(
                            keyword,
                        )]))
                        .into(),
                    })?
                }

                Ok((start, end))
            }
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Keyword(keyword)]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_operator(&mut self) -> Result<(Loc, Loc, Operator), NilangError> {
        match self.assume_next()? {
            Token {
                start,
                end,
                token: TokenType::Operator(operator),
            } => Ok((start, end, operator)),
            Token { start, .. } => Err(NilangError {
                location: CodeLocation::at(start.0, start.1),
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::OpeningParenthesis]))
                    .into(),
            }),
        }
    }
}

pub trait PeekableAssumingIterator: AssumingIterator {
    fn peek_valid(&mut self) -> Result<&Token, NilangError>;
    fn peek_nth_valid(&mut self, n: usize) -> Result<&Token, NilangError>;
}

impl<I: Iterator<Item = Result<Token, NilangError>>> PeekableAssumingIterator for MultiPeekable<I> {
    #[inline]
    fn peek_valid(&mut self) -> Result<&Token, NilangError> {
        self.peek_nth_valid(0)
    }

    fn peek_nth_valid(&mut self, n: usize) -> Result<&Token, NilangError> {
        match self.peek_nth(n) {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e.clone()),
            None => Err(NilangError {
                location: CodeLocation::at(usize::MAX, usize::MAX),
                error: ParserErrors::EndOfInput.into(),
            }),
        }
    }
}
