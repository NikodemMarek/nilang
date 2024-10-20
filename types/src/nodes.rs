#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    FunctionDeclaration {
        name: String,
        parameters: Vec<Box<str>>,
        body: Box<Node>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Node>,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
