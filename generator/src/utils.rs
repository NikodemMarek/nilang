pub fn generate_function(name: &str, code: &[String]) -> Vec<String> {
    let a = [format!(".globl _{name}\n_{name}:")];
    let b = pad_lines(code.iter().chain([String::from("ret\n")].iter()), 4);

    a.iter().chain(b.iter()).map(ToOwned::to_owned).collect()
}

pub fn pad_lines<'a, I: IntoIterator<Item = &'a String>>(lines: I, padding: usize) -> Vec<String> {
    lines
        .into_iter()
        .map(move |line| format!("{:padding$}{}", "", line, padding = padding))
        .collect()
}
