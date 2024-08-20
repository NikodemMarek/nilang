pub fn generate_function(name: &str, code: &[String]) -> Vec<String> {
    let a = [format!(".globl _{name}"), format!("_{name}:")];
    let b = pad_lines(
        code.iter()
            .chain([String::from("ret"), String::new()].iter()),
        4,
    );

    a.iter().chain(b.iter()).map(ToOwned::to_owned).collect()
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

#[cfg(test)]
mod tests {
    #[test]
    fn generate_function() {
        let name = "main";
        let code = Vec::from([String::from("push $6"), String::from("pop %rax")]);
        let result = super::generate_function(name, &code);

        assert_eq!(
            result,
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
    fn pad_lines() {
        let lines = Vec::from([
            String::from("push %rax"),
            String::from("    pop %rax"),
            String::new(),
            String::from("    "),
        ]);
        let result = super::pad_lines(lines.iter(), 4);

        assert_eq!(
            result,
            Vec::from([
                String::from("    push %rax"),
                String::from("        pop %rax"),
                String::new(),
                String::from("        ")
            ])
        );
    }
}
