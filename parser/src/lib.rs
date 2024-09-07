use nilang_lexer::tokens::Token;
use nodes::Node;

pub mod nodes;
mod parsers;

const UNEXPECTED_ERROR: &str = "This does not happen, what the fuck are you doing?";
const UNEXPECTED_END_OF_INPUT_ERROR: &str = "Unexpected end of input!";

pub fn parse(tokens: &[Token]) -> Vec<Node> {
    let mut tokens = tokens.iter().peekable();

    let mut program = Vec::new();
    while tokens.peek().is_some() {
        let node = parsers::parse(&mut program, &mut tokens);
        program.push(node);
    }

    program
}
