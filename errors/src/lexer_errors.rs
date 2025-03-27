#[derive(Debug, Clone)]
pub enum LexerErrors {
    UnexpectedCharacter { char: char, loc: (usize, usize) },
    ExpectedCharacter { char: char, loc: (usize, usize) },
    UnexpectedEndOfFile { loc: (usize, usize) },
}

impl std::fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for LexerErrors {}

impl From<&LexerErrors> for ((usize, usize), (usize, usize), String) {
    fn from(val: &LexerErrors) -> Self {
        match val {
            LexerErrors::UnexpectedCharacter { char, loc } => {
                (*loc, *loc, format!("Unxpected character '{}'", char))
            }
            LexerErrors::ExpectedCharacter { char, loc } => {
                (*loc, *loc, format!("Expected character '{}'", char))
            }
            LexerErrors::UnexpectedEndOfFile { loc } => {
                (*loc, *loc, "Unexpected end of file".into())
            }
        }
    }
}
