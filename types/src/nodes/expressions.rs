use std::collections::HashMap;

use super::{statements::StatementNode, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Primitive(Primitive),
    Parenthesis(Box<ExpressionNode>),
    Object {
        r#type: Type,
        fields: HashMap<Box<str>, ExpressionNode>,
    },
    Operation(Operation),
    VariableReference(Box<str>),
    FieldAccess {
        structure: Box<ExpressionNode>,
        field: Box<str>,
    },
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Boolean(bool),
    Number(f64),
    Char(char),
    String(Box<str>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub a: Box<ExpressionNode>,
    pub b: Box<ExpressionNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Box<str>,
    pub arguments: Box<[ExpressionNode]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub condition: ExpressionNode,
    pub body: Box<[StatementNode]>,
    pub chained: Option<Box<Conditional>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
