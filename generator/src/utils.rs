pub fn generate_function(name: &str, code: &[String]) -> Vec<String> {
    let a = [format!(".globl _{name}"), format!("_{name}:")];
    let b = pad_lines(
        code.iter()
            .chain(space_bottom(&[String::from("ret")]).iter()),
        4,
    );

    a.iter().chain(b.iter()).map(ToOwned::to_owned).collect()
}

/// Saves address of allocation in rax
pub fn generate_allocation(size: u8) -> Vec<String> {
    Vec::from([
        format!("movq ${size}, %rdi"),
        String::from("call malloc"), // TODO: Handle malloc error
    ])
}

pub fn pad_lines<'a, I: IntoIterator<Item = &'a String>>(lines: I, padding: usize) -> Vec<String> {
    lines
        .into_iter()
        .map(move |line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{:padding$}{}", "", line, padding = padding)
            }
        })
        .collect()
}

pub fn space_bottom(lines: &[String]) -> Vec<String> {
    [lines, &[String::new()]].concat()
}

#[cfg(test)]
mod tests {
    use crate::utils::{generate_function, pad_lines, space_bottom};

    #[test]
    fn test_generate_function() {
        assert_eq!(
            generate_function(
                "main",
                &Vec::from([String::from("push $6"), String::from("pop %rax")]),
            ),
            Vec::from([
                String::from(".globl _main"),
                String::from("_main:"),
                String::from("    push $6"),
                String::from("    pop %rax"),
                String::from("    ret"),
                String::new(),
            ])
        );
    }

    #[test]
    fn test_pad_lines() {
        assert_eq!(
            pad_lines(
                Vec::from([
                    String::from("push %rax"),
                    String::from("    pop %rax"),
                    String::new(),
                    String::from("    "),
                ])
                .iter(),
                4,
            ),
            Vec::from([
                String::from("    push %rax"),
                String::from("        pop %rax"),
                String::new(),
                String::from("        "),
            ])
        );
    }

    #[test]
    fn test_space_bottom() {
        assert_eq!(
            space_bottom(&Vec::from([
                String::from("push %rax"),
                String::from("pop %rax"),
            ])),
            Vec::from([
                String::from("push %rax"),
                String::from("pop %rax"),
                String::new(),
            ])
        );
    }
}
