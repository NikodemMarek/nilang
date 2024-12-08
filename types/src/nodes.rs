use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub structures: HashMap<Box<str>, Node>,
    pub functions: HashMap<Box<str>, Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    FunctionDeclaration {
        name: Box<str>,
        parameters: HashMap<Box<str>, Box<str>>,
        return_type: Box<str>,
        body: Box<Node>,
    },
    FunctionCall {
        name: Box<str>,
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
        name: Box<str>,
        r#type: Box<str>,
        value: Box<Node>,
    },
    VariableReference(Box<str>),
    Return(Box<Node>),
    Structure {
        name: Box<str>,
        fields: HashMap<Box<str>, Box<str>>, // name, type
    },
    Object {
        structure: Box<str>,
        fields: HashMap<Box<str>, Node>,
    },
    FieldAccess {
        structure: Box<Node>,
        field: Box<str>,
    },
}

#[derive(Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operator::")?;
        match self {
            Operator::Add => write!(f, "Add"),
            Operator::Subtract => write!(f, "Subtract"),
            Operator::Multiply => write!(f, "Multiply"),
            Operator::Divide => write!(f, "Divide"),
            Operator::Modulo => write!(f, "Modulo"),
        }
    }
}
