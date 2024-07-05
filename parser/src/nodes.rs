#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub program: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(f64),
    Operation {
        operator: Operator,
        a: Box<Node>,
        b: Box<Node>,
    },
    ParenthesisTerminator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
