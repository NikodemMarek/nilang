use crate::NilangErrorKind;

#[derive(Debug, Clone)]
pub enum LexerErrors {
    UnexpectedCharacter(char),
    ExpectedCharacter(char),
    UnexpectedEndOfFile,
}

impl std::fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerErrors::UnexpectedCharacter(char) => {
                write!(f, "Unxpected character '{}'", char)
            }
            LexerErrors::ExpectedCharacter(char) => {
                write!(f, "Expected character '{}'", char)
            }
            LexerErrors::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
        }
    }
}

impl From<LexerErrors> for NilangErrorKind {
    fn from(val: LexerErrors) -> Self {
        NilangErrorKind::LexerError(val)
    }
}

impl std::error::Error for LexerErrors {}
