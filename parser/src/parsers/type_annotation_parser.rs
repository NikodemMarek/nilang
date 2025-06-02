use errors::NilangError;
use nilang_types::{nodes::Type, tokens::TokenType, Localizable};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_type_annotation<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<Localizable<Type>, NilangError> {
    tokens.assume(TokenType::Colon)?;

    let r#type = tokens.assume_identifier()?;

    Ok(Localizable::new(
        r#type.location,
        parse_type(&r#type.object),
    ))
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
    use nilang_types::{nodes::Type, tokens::TokenType, Localizable};

    use crate::parsers::type_annotation_parser::parse_type_annotation;

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_type_annotation(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Colon)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()))),
                ]
                .into_iter()
                .peekable()
            )
            .unwrap()
            .object,
            Type::Int,
        );
    }
}
