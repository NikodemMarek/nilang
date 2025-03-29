use std::{collections::HashMap, fmt::Debug};

pub type Parameter = (Box<str>, Type);

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Box<str>,
    pub parameters: Box<[Parameter]>,
    pub return_type: Type,
    pub body: Box<[StatementNode]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureDeclaration {
    pub name: Box<str>,
    pub fields: HashMap<Box<str>, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Number(f64),
    Char(char),
    String(Box<str>),
    Object {
        r#type: Type,
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

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    VariableDeclaration {
        name: Box<str>,
        r#type: Type,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Void,
    Int,
    Char,
    Object(Box<str>),
}
