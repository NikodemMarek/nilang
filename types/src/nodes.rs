use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: HashMap<Box<str>, FunctionDeclaration>,
    pub structures: HashMap<Box<str>, Structure>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Box<str>,
    pub parameters: Box<[Parameter]>,
    pub return_type: Box<str>,
    pub body: Box<[StatementNode]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    pub name: Box<str>,
    pub fields: HashMap<Box<str>, Box<str>>, // name, type
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Number(f64),
    Char(char),
    String(Box<str>),
    Object {
        r#type: Box<str>,
        fields: HashMap<Box<str>, ExpressionNode>,
    },
    Operation {
        operator: Operator,
        a: Box<ExpressionNode>,
        b: Box<ExpressionNode>,
    },
    VariableReference(Box<str>),
    FieldAccess {
        structure: Box<ExpressionNode>,
        field: Box<str>,
    },
    FunctionCall(FunctionCall),
}

pub type Parameter = (Box<str>, Box<str>);

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    VariableDeclaration {
        name: Box<str>,
        r#type: Box<str>,
        value: Box<ExpressionNode>,
    },
    Return(Box<ExpressionNode>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Box<str>,
    pub arguments: Box<[ExpressionNode]>,
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
