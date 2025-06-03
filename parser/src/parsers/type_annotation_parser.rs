use errors::NilangError;
use nilang_types::{
    nodes::{Type, TypedIdentifier},
    tokens::TokenType,
    Localizable as L,
};

use crate::assuming_iterator::PeekableAssumingIterator;

pub fn parse_typed_identifier<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<TypedIdentifier, NilangError> {
    let name = tokens.assume_identifier()?;
    let r#type = parse_type_annotation(tokens)?;
    Ok((name, r#type))
}

pub fn parse_type_annotation<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<Type>, NilangError> {
    tokens.assume(TokenType::Colon)?;

    let r#type = tokens.assume_identifier()?;

    Ok(L::new(r#type.location, parse_type(&r#type.object)))
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
    use nilang_types::{nodes::Type, tokens::TokenType, Localizable as L};

    use crate::parsers::type_annotation_parser::parse_type_annotation;

    #[test]
    fn test_parse_structure() {
        assert_eq!(
            parse_type_annotation(
                &mut [
                    Ok(L::irrelevant(TokenType::Colon)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()))),
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
