use colored::Colorize;
pub use generator_errors::GeneratorErrors;
pub use lexer_errors::LexerErrors;
pub use parser_errors::ParserErrors;

mod generator_errors;
mod lexer_errors;
mod parser_errors;

#[derive(Debug, Clone)]
pub struct NilangError {
    pub code: String,

    // Range
    pub start: (usize, usize),
    pub end: (usize, usize),

    pub message: String,
}

impl std::fmt::Display for NilangError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let context = highlight_with_context(&self.code, self.start, self.end);
        write!(
            f,
            "{}\n{}",
            format_error_message(self.start, self.end, &self.message),
            context
        )
    }
}

impl std::error::Error for NilangError {}

fn format_error_message(
    (start_line, start_column): (usize, usize),
    _: (usize, usize),
    message: &str,
) -> String {
    format!("[{}:{}] {}", start_line, start_column, message)
        .as_str()
        .red()
        .to_string()
}

fn highlight_with_context(
    code: &str,
    (start_line, start_column): (usize, usize),
    (end_line, end_column): (usize, usize),
) -> String {
    let first_line = if start_line < 3 { 0 } else { start_line - 3 };
    let window = code
        .lines()
        .skip(first_line)
        .take(if start_line < 3 {
            end_line + 3
        } else {
            end_line - start_line + 6
        })
        .enumerate()
        .map(|(i, line)| {
            let line = if i >= start_line && i <= end_line {
                if i == start_line && start_line == end_line {
                    let before_error = line.chars().take(start_column).collect::<String>();
                    let error = line
                        .chars()
                        .skip(start_column)
                        .take(end_column - start_column + 1)
                        .collect::<String>();
                    let after_error = line.chars().skip(end_column + 1).collect::<String>();
                    format!("{}{}{}", before_error, error.red().underline(), after_error)
                } else if i == start_line {
                    let before_error = line.chars().clone().take(start_column).collect::<String>();
                    let error = line.chars().clone().skip(start_column).collect::<String>();
                    format!("{}{}", before_error, error.red().underline())
                } else if i == end_line {
                    let chars = line.chars().skip(start_column);
                    let error = chars.clone().take(end_column).collect::<String>();
                    let after_error = chars.clone().skip(end_column + 1).collect::<String>();
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
