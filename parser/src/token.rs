#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub program: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Operator {
        operator: Operator,
        a: Box<Token>,
        b: Box<Token>,
    },
    Space,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
