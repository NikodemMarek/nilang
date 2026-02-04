use nilang_types::nodes::expressions::Operator;

pub(super) fn is_preceeding(a: Operator, b: Operator) -> bool {
    precendence_score(a) > precendence_score(b)
}

fn precendence_score(operator: Operator) -> u8 {
    match operator {
        Operator::Add | Operator::Subtract => 0,
        Operator::Multiply | Operator::Divide | Operator::Modulo => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nilang_types::nodes::expressions::Operator;

    #[test]
    fn test_precendence_score() {
        assert_eq!(precendence_score(Operator::Add), 0);
        assert_eq!(precendence_score(Operator::Subtract), 0);
        assert_eq!(precendence_score(Operator::Multiply), 1);
        assert_eq!(precendence_score(Operator::Divide), 1);
        assert_eq!(precendence_score(Operator::Modulo), 1);
    }

    #[test]
    fn test_is_preceeding() {
        // Test basic arithmetic
        assert!(is_preceeding(Operator::Multiply, Operator::Add));
        assert!(is_preceeding(Operator::Divide, Operator::Subtract));
        assert!(is_preceeding(Operator::Modulo, Operator::Add));
    }
}
