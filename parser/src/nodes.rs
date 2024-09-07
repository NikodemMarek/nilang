#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    FunctionDeclaration {
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
    VariableDeclaration {
        name: String,
        value: Box<Node>,
    },
    VariableReference(String),
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
