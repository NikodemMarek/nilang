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
    Boolean(bool),
    Number(f64),
    Char(char),
    String(Box<str>),
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
pub enum StatementNode {
    VariableDeclaration {
        name: Box<str>,
        r#type: Type,
        value: Box<ExpressionNode>,
    },
    VariableAssignment {
        name: Box<str>,
        value: Box<ExpressionNode>,
    },
    Return(Box<ExpressionNode>),
    FunctionCall(FunctionCall),
    Conditional(Conditional),
    WhileLoop {
        condition: ExpressionNode,
        body: Box<[StatementNode]>,
    },
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Void,
    Bool,
    Int,
    Char,
    String,
    Object(Box<str>),
}
