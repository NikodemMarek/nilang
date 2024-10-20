use errors::{LexerErrors, ParserErrors};
use nilang_types::{nodes::Node, tokens::Token};

mod parsers;

pub fn parse(
    tokens: impl Iterator<Item = Result<Token, LexerErrors>>,
) -> Result<Node, ParserErrors> {
    let mut tokens = tokens.peekable();

    let mut program = Vec::new();
    while tokens.peek().is_some() {
        let node = parsers::parse(&mut tokens)?;
        program.push(node);
    }

    Ok(Node::Program(program))
}
