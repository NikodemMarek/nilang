use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Box<str>,
    pub parameters: Box<[Parameter]>,
    pub return_type: Box<str>,
    pub body: Box<[StatementNode]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureDeclaration {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
