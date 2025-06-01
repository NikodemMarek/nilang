use colored::Colorize;
pub use generator_errors::GeneratorErrors;
pub use lexer_errors::LexerErrors;
pub use parser_errors::ParserErrors;
pub use transformer_errors::TransformerErrors;

mod generator_errors;
mod lexer_errors;
mod parser_errors;
mod transformer_errors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeLocation(usize, usize, usize, usize);
impl CodeLocation {
    pub fn at(line: usize, char: usize) -> Self {
        Self(line, char, line, char)
    }
    pub fn range(line_from: usize, char_from: usize, line_to: usize, char_to: usize) -> Self {
        Self(line_from, char_from, line_to, char_to)
    }
}

#[derive(Debug, Clone)]
pub struct NilangError {
    pub location: CodeLocation,
    pub error: NilangErrorKind,
}

impl NilangError {
    pub fn format_error(&self, code: &str) -> String {
        let context = highlight_with_context(code, self.location);
        format!(
            "{}\n{}",
            format_error_message(self.location, &self.error),
            context
        )
    }
}

impl std::fmt::Display for NilangError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format_error_message(self.location, &self.error))
    }
}

impl std::error::Error for NilangError {}

#[derive(Debug, Clone)]
pub enum NilangErrorKind {
    LexerError(LexerErrors),
    ParserError(ParserErrors),
    // TODO: GeneratorError, TransformerError
}

impl std::fmt::Display for NilangErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NilangErrorKind::LexerError(err) => write!(f, "Lexer: {}", err),
            NilangErrorKind::ParserError(err) => write!(f, "Parser: {}", err),
        }
    }
}

fn format_error_message(
    CodeLocation(line_from, _, line_to, _): CodeLocation,
    error: &NilangErrorKind,
) -> String {
    format!("[{}:{}] {}", line_from, line_to, error)
        .as_str()
        .red()
        .to_string()
}

fn highlight_with_context(
    code: &str,
    CodeLocation(line_from, char_from, line_to, char_to): CodeLocation,
) -> String {
    let first_line = if line_from < 3 { 0 } else { line_from - 3 };
    let window = code
        .lines()
        .skip(first_line)
        .take(if line_from < 3 {
            line_to + 3
        } else {
            line_to - line_from + 6
        })
        .enumerate()
        .map(|(i, line)| {
            let line = if i >= line_from && i <= line_to {
                if i == line_from && line_from == line_to {
                    let before_error = line.chars().take(char_from).collect::<String>();
                    let error = line
                        .chars()
                        .skip(char_from)
                        .take(char_to - char_from + 1)
                        .collect::<String>();
                    let after_error = line.chars().skip(char_to + 1).collect::<String>();
                    format!("{}{}{}", before_error, error.red().underline(), after_error)
                } else if i == line_from {
                    let before_error = line.chars().clone().take(char_from).collect::<String>();
                    let error = line.chars().clone().skip(char_from).collect::<String>();
                    format!("{}{}", before_error, error.red().underline())
                } else if i == line_to {
                    let chars = line.chars().skip(char_from);
                    let error = chars.clone().take(char_to).collect::<String>();
                    let after_error = chars.clone().skip(char_to + 1).collect::<String>();
                    format!("{}{}", error.red().underline(), after_error)
                } else {
                    format!("{}", line.red().underline())
                }
            } else {
                line.to_owned()
            };
            format!("{:>4} | {}", first_line + i + 1, line)
        })
        .collect::<Vec<String>>()
        .join("\n");

    window
}
