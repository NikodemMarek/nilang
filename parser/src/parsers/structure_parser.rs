use std::collections::HashMap;

use errors::{CodeLocation, NilangError, ParserErrors};
use nilang_types::{
    nodes::statements::StructureDeclaration,
    tokens::{Keyword, Token, TokenType},
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::type_annotation_parser::parse_type_annotation;

pub fn parse_structure<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<StructureDeclaration, NilangError> {
    tokens.assume_keyword(Keyword::Structure)?;

    let (_, _, name) = tokens.assume_identifier()?;

    tokens.assume(TokenType::OpeningBrace)?;

    let mut fields = HashMap::new();

    loop {
        let (_, _, name) = tokens.assume_identifier()?;
        let r#type = parse_type_annotation(tokens)?;

        fields.insert(name, r#type);

        match tokens.peek_valid()? {
            Token {
                token: TokenType::Comma,
                ..
            } => {
                let _ = tokens.assume(TokenType::Comma);

                if let Token {
                    token: TokenType::ClosingBrace,
                    ..
                } = tokens.peek_valid()?
                {
                    break;
                }
            }
            Token {
                token: TokenType::ClosingBrace,
                ..
            } => break,
            _ => {
                return Err(NilangError {
                    location: {
                        let start = tokens.peek_valid()?.start;
                        CodeLocation::at(start.0, start.1)
                    },
                    error: ParserErrors::ExpectedTokens(vec![
                        TokenType::Comma,
                        TokenType::ClosingBrace,
                    ])
                    .into(),
                });
            }
        }
    }

    tokens.assume(TokenType::ClosingBrace)?;

    Ok(StructureDeclaration { name, fields })
}

#[cfg(test)]
mod test {
    use nilang_types::{
        nodes::{statements::StructureDeclaration, Type},
        tokens::{Keyword, Token, TokenType},
    };

    use crate::{multi_peekable::MultiPeekable, parsers::structure_parser::parse_structure};

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_structure(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Structure,),
                        start: (0, 0,),
                        end: (0, 1,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("Test".into(),),
                        start: (0, 3,),
                        end: (0, 6,),
                    },),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 8,),
                        end: (0, 8,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("test_field".into(),),
                        start: (1, 4,),
                        end: (1, 13,),
                    },),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (1, 14,),
                        end: (1, 14,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("int".into(),),
                        start: (1, 16,),
                        end: (1, 18,),
                    },),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (1, 19,),
                        end: (1, 19,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("test_field2".into(),),
                        start: (2, 4,),
                        end: (2, 14,),
                    },),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (2, 15,),
                        end: (2, 15,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("int".into(),),
                        start: (2, 17,),
                        end: (2, 19,),
                    },),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (3, 0,),
                        end: (3, 0,),
                    },),
                ]
                .into_iter()
            ))
            .unwrap(),
            StructureDeclaration {
                name: "Test".into(),
                fields: [
                    ("test_field".into(), Type::Int),
                    ("test_field2".into(), Type::Int),
                ]
                .into(),
            }
        );

        assert_eq!(
            parse_structure(&mut MultiPeekable::new(
                [
                    Ok(Token {
                        token: TokenType::Keyword(Keyword::Structure,),
                        start: (0, 0,),
                        end: (0, 1,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("Test".into()),
                        start: (0, 3,),
                        end: (0, 6,),
                    },),
                    Ok(Token {
                        token: TokenType::OpeningBrace,
                        start: (0, 8,),
                        end: (0, 8,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("test_field".into()),
                        start: (1, 4,),
                        end: (1, 13,),
                    },),
                    Ok(Token {
                        token: TokenType::Colon,
                        start: (1, 14,),
                        end: (1, 14,),
                    },),
                    Ok(Token {
                        token: TokenType::Identifier("int".into()),
                        start: (1, 16,),
                        end: (1, 18,),
                    },),
                    Ok(Token {
                        token: TokenType::Comma,
                        start: (1, 19,),
                        end: (1, 19,),
                    },),
                    Ok(Token {
                        token: TokenType::ClosingBrace,
                        start: (2, 0,),
                        end: (2, 0,),
                    },),
                ]
                .into_iter()
            ))
            .unwrap(),
            StructureDeclaration {
                name: "Test".into(),
                fields: [("test_field".into(), Type::Int)].into(),
            },
        );
    }
}
