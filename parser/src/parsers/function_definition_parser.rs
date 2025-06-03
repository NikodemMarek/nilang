use errors::NilangError;
use nilang_types::{
    nodes::{FunctionBody, FunctionDeclaration},
    tokens::{Keyword, TokenType},
    Localizable as L, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    parameters_parser::parse_parameters, parse_statement,
    type_annotation_parser::parse_type_annotation,
};

pub fn parse_function_definition<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<FunctionDeclaration>, NilangError> {
    tokens.assume_keyword(Keyword::Function)?;

    let name = tokens.assume_identifier()?;

    let parameters = parse_parameters(tokens)?;

    let return_type = parse_type_annotation(tokens)?;

    let body = parse_function_body(tokens)?;

    Ok(L::new(
        Location::between(&name.location, &body.location),
        FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        },
    ))
}

fn parse_function_body<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<L<FunctionBody>, NilangError> {
    let start = tokens.assume(TokenType::OpeningBrace)?;

    let mut body = Vec::new();
    while let L {
        object: TokenType::Keyword(_) | TokenType::Identifier(_),
        ..
    } = tokens.peek_valid()?
    {
        body.push(parse_statement(tokens)?);
    }

    let end = tokens.assume(TokenType::ClosingBrace)?;

    Ok(L::new(
        Location::between(&start, &end),
        body.into_boxed_slice(),
    ))
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionDeclaration, StatementNode, Type},
        tokens::{Keyword, TokenType},
        Localizable as L,
    };

    use super::parse_function_definition;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            parse_function_definition(
                &mut [
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Function),)),
                    Ok(L::irrelevant(TokenType::Identifier("main".into()),)),
                    Ok(L::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(L::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(L::irrelevant(TokenType::Colon,)),
                    Ok(L::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(L::irrelevant(TokenType::OpeningBrace,)),
                    Ok(L::irrelevant(TokenType::Keyword(Keyword::Return),)),
                    Ok(L::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(L::irrelevant(TokenType::Semicolon,)),
                    Ok(L::irrelevant(TokenType::ClosingBrace,))
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap()
            .object,
            FunctionDeclaration {
                name: L::irrelevant("main".into()),
                parameters: L::irrelevant([].into()),
                return_type: L::irrelevant(Type::Int),
                body: L::irrelevant(Box::new([L::irrelevant(StatementNode::Return(Box::new(
                    L::irrelevant(ExpressionNode::Number(6.))
                )))])),
            }
        );
    }
}
