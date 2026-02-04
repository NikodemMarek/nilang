use nilang_types::nodes::expressions::{Arithmetic, Operator};

pub(super) fn is_preceeding(a: Operator, b: Operator) -> bool {
    precendence_score(a) > precendence_score(b)
}

fn precendence_score(operator: Operator) -> u8 {
    match operator {
        Operator::Arithmetic(operator) => match operator {
            Arithmetic::Add | Arithmetic::Subtract => 0,
            Arithmetic::Multiply | Arithmetic::Divide | Arithmetic::Modulo => 1,
        },
        Operator::Boolean(_) => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nilang_types::nodes::expressions::{Boolean, Operator};

    #[test]
    fn test_is_preceeding() {
        // arithmetic
        assert!(is_preceeding(
            Operator::Arithmetic(Arithmetic::Multiply),
            Operator::Arithmetic(Arithmetic::Add)
        ));
        assert!(is_preceeding(
            Operator::Arithmetic(Arithmetic::Divide),
            Operator::Arithmetic(Arithmetic::Subtract)
        ));
        assert!(is_preceeding(
            Operator::Arithmetic(Arithmetic::Modulo),
            Operator::Arithmetic(Arithmetic::Add)
        ));

        // arithmetic + booleans
        assert!(is_preceeding(
            Operator::Boolean(Boolean::Equal),
            Operator::Arithmetic(Arithmetic::Add)
        ));
        assert!(is_preceeding(
            Operator::Boolean(Boolean::Equal),
            Operator::Arithmetic(Arithmetic::Multiply)
        ));
    }
}
