pub fn pad_lines<'a, I: IntoIterator<Item = &'a Box<str>>>(
    lines: I,
    padding: usize,
) -> Vec<Box<str>> {
    lines
        .into_iter()
        .map(move |line| {
            if line.is_empty() {
                String::new().into()
            } else {
                format!("{:padding$}{}", "", line, padding = padding).into()
            }
        })
        .collect()
}

pub fn space_bottom(lines: &[Box<str>]) -> Vec<Box<str>> {
    [lines, &["".into()]].concat()
}

#[cfg(test)]
mod tests {
    use crate::utils::{pad_lines, space_bottom};

    #[test]
    fn test_pad_lines() {
        assert_eq!(
            pad_lines(
                Vec::from([
                    "push %rax".into(),
                    "    pop %rax".into(),
                    "".into(),
                    "    ".into(),
                ])
                .iter(),
                4,
            ),
            Vec::from([
                "    push %rax".into(),
                "        pop %rax".into(),
                "".into(),
                "        ".into(),
            ])
        );
    }

    #[test]
    fn test_space_bottom() {
        assert_eq!(
            space_bottom(&Vec::from(["push %rax".into(), "pop %rax".into(),])),
            Vec::from(["push %rax".into(), "pop %rax".into(), "".into(),])
        );
    }
}
