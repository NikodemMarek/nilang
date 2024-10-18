use crate::lex;

#[test]
fn convert_whitespace() {
    let mut iter = lex(" \n\t \t\t\n   \n\t \t\t\n ");
    assert!(iter.next().is_none());
}
