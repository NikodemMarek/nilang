use errors::ParserErrors;

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_type_annotation<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Box<str>, ParserErrors> {
    tokens.assume_colon()?;

    let (_, _, r#type) = tokens.assume_identifier()?;

    Ok(r#type)
}

#[cfg(test)]
mod test {
    use nilang_types::tokens::{Token, TokenType};

    use crate::parsers::type_annotation_parser::parse_type_annotation;

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_type_annotation(
                &mut [
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
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            "int".into(),
        );
    }
}
