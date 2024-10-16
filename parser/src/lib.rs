use nilang_types::{nodes::Node, tokens::Token};

mod parsers;

pub fn parse(tokens: &[Token]) -> eyre::Result<Vec<Node>> {
    let mut tokens = tokens.iter().peekable();

    let mut program = Vec::new();
    while tokens.peek().is_some() {
        let node = parsers::parse(&mut program, &mut tokens)?;
        program.push(node);
    }

    Ok(program)
}
