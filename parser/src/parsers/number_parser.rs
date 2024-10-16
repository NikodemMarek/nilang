use errors::ParserErrors;
use nilang_types::{
    nodes::Node,
    tokens::{Token, TokenType},
};

pub fn parse_number(
    Token {
        token,
        value,
        start,
        end,
    }: &Token,
) -> eyre::Result<Node> {
    if let TokenType::Number = token {
        Ok(Node::Number(match value.parse() {
            Ok(parsed) => parsed,
            Err(_) => Err(ParserErrors::NotANumber {
                from: *start,
                to: *end,
            })?,
        }))
    } else {
        Err(ParserErrors::ThisNeverHappens)?
    }
}

#[cfg(test)]
mod tests {
    use nilang_types::{
        nodes::Node,
        tokens::{Token, TokenType},
    };

    use crate::parsers::number_parser::parse_number;

    #[test]
    fn parse_numbers() {
        assert_eq!(
            parse_number(&Token {
                token: TokenType::Number,
                value: "54".to_string(),
                start: (0, 0),
                end: (0, 2),
            })
            .unwrap(),
            Node::Number(54.)
        );
        assert_eq!(
            parse_number(&Token {
                token: TokenType::Number,
                value: "6.".to_string(),
                start: (0, 0),
                end: (0, 2),
            })
            .unwrap(),
            Node::Number(6.)
        );
        assert_eq!(
            parse_number(&Token {
                token: TokenType::Number,
                value: ".2".to_string(),
                start: (0, 0),
                end: (0, 2),
            })
            .unwrap(),
            Node::Number(0.2)
        );
        assert_eq!(
            parse_number(&Token {
                token: TokenType::Number,
                value: "8.5".to_string(),
                start: (0, 0),
                end: (0, 2),
            })
            .unwrap(),
            Node::Number(8.5)
        );
    }
}
