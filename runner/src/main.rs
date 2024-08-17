use std::path::Path;

use nilang_generator::compile;

fn main() {
    let hw = compile(
        r#"
    fn main() {
        rt 5 + 2
    }
    "#,
    );

    let path = Path::new("test.asm");
    std::fs::write(path, hw).unwrap();
}
