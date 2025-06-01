use errors::NilangError;
use nilang_types::{nodes::Type, tokens::TokenType};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_type_annotation<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Type, NilangError> {
    tokens.assume(TokenType::Colon)?;

    let (_, _, r#type) = tokens.assume_identifier()?;

    Ok(parse_type(&r#type))
}

pub fn parse_type(r#type: &str) -> Type {
    match r#type.to_string().as_str() {
        "void" => Type::Void,
        "int" => Type::Int,
        "char" => Type::Char,
        "string" => Type::String,
        r#type => Type::Object(r#type.into()),
    }
}

#[cfg(test)]
mod test {
    use nilang_types::{
        nodes::Type,
        tokens::{Token, TokenType},
    };

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
            Type::Int,
        );
    }
}
