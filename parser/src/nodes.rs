#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<Node>,
    },
    Number(f64),
    Operation {
        operator: Operator,
        a: Box<Node>,
        b: Box<Node>,
    },
    Scope(Vec<Node>),
    Return(Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
