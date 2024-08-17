use crate::convert;

#[test]
fn convert_whitespaces() {
    assert_eq!(convert(" \n\t \t\t\n   \n\t \t\t\n "), vec![],)
}
