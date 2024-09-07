use nilang_lexer::tokens::{Token, TokenType};

use crate::{nodes::Node, UNEXPECTED_ERROR};

pub fn parse_number(
    Token {
        token,
        value,
        start,
        end,
    }: &Token,
) -> Node {
    if let TokenType::Number = token {
        Node::Number(
            value
                .parse()
                .unwrap_or_else(|_| panic!("[{start}-{end}] Invalid number: \"{value}\"")),
        )
    } else {
        panic!("{}", UNEXPECTED_ERROR);
    }
}

#[cfg(test)]
mod tests {
    use nilang_lexer::tokens::{Token, TokenType};

    use crate::{nodes::Node, parse};

    #[test]
    fn parse_numbers() {
        assert_eq!(
            &parse(&[Token {
                token: TokenType::Number,
                value: "54".to_string(),
                start: 0,
                end: 2,
            }]),
            &[Node::Number(54.)]
        );
        assert_eq!(
            &parse(&[Token {
                token: TokenType::Number,
                value: "6.".to_string(),
                start: 0,
                end: 2,
            }]),
            &[Node::Number(6.)]
        );
        assert_eq!(
            &parse(&[Token {
                token: TokenType::Number,
                value: ".2".to_string(),
                start: 0,
                end: 2,
            }]),
            &[Node::Number(0.2)]
        );
        assert_eq!(
            &parse(&[Token {
                token: TokenType::Number,
                value: "8.5".to_string(),
                start: 0,
                end: 2,
            }]),
            &[Node::Number(8.5)]
        );
    }
}
