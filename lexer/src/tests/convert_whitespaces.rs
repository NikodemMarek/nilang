use crate::lex;

#[test]
fn convert_whitespaces() {
    assert_eq!(lex(" \n\t \t\t\n   \n\t \t\t\n "), vec![],)
}
