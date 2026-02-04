use std::collections::HashMap;

use super::{statements::StatementNode, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Primitive(Primitive),
    VariableReference(Box<str>),
    FieldAccess {
        structure: Box<ExpressionNode>,
        field: Box<str>,
    },
    FunctionCall(FunctionCall),
    Parenthesis(Box<ExpressionNode>),
    Operation(Operation),
    Object {
        r#type: Type,
        fields: HashMap<Box<str>, ExpressionNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Boolean(bool),
    Number(f64),
    Char(char),
    String(Box<str>),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub a: Box<ExpressionNode>,
    pub b: Box<ExpressionNode>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Arithmetic(Arithmetic),
    Boolean(Boolean),
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum Arithmetic {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum Boolean {
    #[default]
    Equal,
    NotEqual,
    Less,
    More,
    LessOrEqual,
    MoreOrEqual,
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

impl Default for ExpressionNode {
    fn default() -> Self {
        Self::Primitive(Primitive::default())
    }
}

impl Default for Primitive {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl Default for Operator {
    fn default() -> Self {
        Self::Arithmetic(Default::default())
    }
}
