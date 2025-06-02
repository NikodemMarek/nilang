use std::iter::Peekable;

use errors::{NilangError, ParserErrors};
use nilang_types::{
    nodes::Operator,
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

pub trait AssumingIterator: Iterator {
    fn assume_next(&mut self) -> Result<Localizable<TokenType>, NilangError>;

    fn assume(&mut self, token: TokenType) -> Result<Location, NilangError>;
    fn assume_literal(&mut self) -> Result<Localizable<Box<str>>, NilangError>;
    fn assume_identifier(&mut self) -> Result<Localizable<Box<str>>, NilangError>;
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<Location, NilangError>;
    fn assume_operator(&mut self) -> Result<Localizable<Operator>, NilangError>;
}

impl<I: Iterator<Item = Result<Localizable<TokenType>, NilangError>>> AssumingIterator for I {
    #[inline]
    fn assume(&mut self, token: TokenType) -> Result<Location, NilangError> {
        match self.assume_next()? {
            Localizable {
                location,
                object: t,
            } if t == token => Ok(location),
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([token])).into(),
            }),
        }
    }

    #[inline]
    fn assume_next(&mut self) -> Result<Localizable<TokenType>, NilangError> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e.clone()),
            None => Err(NilangError {
                location: Location::at(usize::MAX, usize::MAX),
                error: ParserErrors::EndOfInput.into(),
            }),
        }
    }

    #[inline]
    fn assume_literal(&mut self) -> Result<Localizable<Box<str>>, NilangError> {
        match self.assume_next()? {
            Localizable {
                object: TokenType::Literal(value),
                location,
            } => Ok(Localizable::new(location, value)),
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Literal("".into())]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_identifier(&mut self) -> Result<Localizable<Box<str>>, NilangError> {
        match self.assume_next()? {
            Localizable {
                object: TokenType::Identifier(value),
                location,
            } => Ok(Localizable::new(location, value)),
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Identifier("".into())]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_keyword(&mut self, keyword: Keyword) -> Result<Location, NilangError> {
        match self.assume_next()? {
            Localizable {
                object: TokenType::Keyword(value),
                location,
            } => {
                if value != keyword {
                    Err(NilangError {
                        location: Location::at(0, 1),
                        error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Keyword(
                            keyword,
                        )]))
                        .into(),
                    })?
                }

                Ok(location)
            }
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::Keyword(keyword)]))
                    .into(),
            }),
        }
    }

    #[inline]
    fn assume_operator(&mut self) -> Result<Localizable<Operator>, NilangError> {
        match self.assume_next()? {
            Localizable {
                object: TokenType::Operator(operator),
                location,
            } => Ok(Localizable::new(location, operator)),
            Localizable { location, .. } => Err(NilangError {
                location,
                error: ParserErrors::ExpectedTokens(Vec::from([TokenType::OpeningParenthesis]))
                    .into(),
            }),
        }
    }
}

pub trait PeekableAssumingIterator: AssumingIterator {
    fn peek_valid(&mut self) -> Result<&Localizable<TokenType>, NilangError>;
}

impl<I: Iterator<Item = Result<Localizable<TokenType>, NilangError>>> PeekableAssumingIterator
    for Peekable<I>
{
    #[inline]
    fn peek_valid(&mut self) -> Result<&Localizable<TokenType>, NilangError> {
        match self.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e.clone()),
            None => Err(NilangError {
                location: Location::at(usize::MAX, usize::MAX),
                error: ParserErrors::EndOfInput.into(),
            }),
        }
    }
}
