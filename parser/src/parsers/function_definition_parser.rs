use errors::NilangError;
use nilang_types::{
    nodes::FunctionDeclaration,
    tokens::{Keyword, TokenType},
    Localizable, Location,
};

use crate::assuming_iterator::PeekableAssumingIterator;

use super::{
    parameter_list_parser::parse_parameter_list, parse_statement,
    type_annotation_parser::parse_type_annotation,
};

pub fn parse_function_definition<I: PeekableAssumingIterator>(
    tokens: &mut I,
) -> Result<FunctionDeclaration, NilangError> {
    tokens.assume_keyword(Keyword::Function)?;

    let name = tokens.assume_identifier()?;

    let parameters = parse_parameter_list(tokens)?;

    let return_type = parse_type_annotation(tokens)?;

    let start = tokens.assume(TokenType::OpeningBrace)?;

    {
        let mut program = Vec::new();

        loop {
            match tokens.peek_valid()? {
                Localizable {
                    object: TokenType::ClosingBrace,
                    location: end,
                } => {
                    let location = Location::between(&start, end);
                    let body = Localizable::new(location, program.into_boxed_slice());

                    tokens.assume(TokenType::ClosingBrace)?;

                    return Ok(FunctionDeclaration {
                        name,
                        parameters,
                        return_type,
                        body,
                    });
                }
                Localizable { .. } => {
                    program.push(parse_statement(tokens)?);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::{ExpressionNode, FunctionDeclaration, StatementNode, Type},
        tokens::{Keyword, TokenType},
        Localizable,
    };

    use super::parse_function_definition;

    #[test]
    fn test_parse_function_definition() {
        assert_eq!(
            parse_function_definition(
                &mut [
                    Ok(Localizable::irrelevant(TokenType::Keyword(
                        Keyword::Function
                    ),)),
                    Ok(Localizable::irrelevant(TokenType::Identifier(
                        "main".into()
                    ),)),
                    Ok(Localizable::irrelevant(TokenType::OpeningParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::ClosingParenthesis,)),
                    Ok(Localizable::irrelevant(TokenType::Colon,)),
                    Ok(Localizable::irrelevant(TokenType::Identifier("int".into()),)),
                    Ok(Localizable::irrelevant(TokenType::OpeningBrace,)),
                    Ok(Localizable::irrelevant(TokenType::Keyword(Keyword::Return),)),
                    Ok(Localizable::irrelevant(TokenType::Literal("6".into()),)),
                    Ok(Localizable::irrelevant(TokenType::Semicolon,)),
                    Ok(Localizable::irrelevant(TokenType::ClosingBrace,))
                ]
                .into_iter()
                .peekable(),
            )
            .unwrap(),
            FunctionDeclaration {
                name: Localizable::irrelevant("main".into()),
                parameters: Localizable::irrelevant([].into()),
                return_type: Localizable::irrelevant(Type::Int),
                body: Localizable::irrelevant(Box::new([Localizable::irrelevant(
                    StatementNode::Return(Box::new(Localizable::irrelevant(
                        ExpressionNode::Number(6.)
                    )))
                )])),
            }
        );
    }
}
