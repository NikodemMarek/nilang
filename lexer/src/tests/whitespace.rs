use crate::lex;

#[test]
fn convert_whitespace() {
    assert_eq!(&lex(" \n\t \t\t\n   \n\t \t\t\n ").unwrap(), &[],)
}
