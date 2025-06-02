use std::{collections::HashMap, fmt::Debug};

use crate::Localizable;

pub type Parameter = (Localizable<Box<str>>, Localizable<Type>);

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Localizable<Box<str>>,
    pub parameters: Localizable<Box<[Parameter]>>,
    pub return_type: Localizable<Type>,
    pub body: Localizable<Box<[Localizable<StatementNode>]>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureDeclaration {
    pub name: Localizable<Box<str>>,
    pub fields: Localizable<HashMap<Localizable<Box<str>>, Localizable<Type>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Number(f64),
    Char(char),
    String(Box<str>),
    Object {
        r#type: Localizable<Type>,
        fields: Localizable<HashMap<Localizable<Box<str>>, Localizable<ExpressionNode>>>,
    },
    Operation {
        operator: Localizable<Operator>,
        a: Box<Localizable<ExpressionNode>>,
        b: Box<Localizable<ExpressionNode>>,
    },
    VariableReference(Box<str>),
    FieldAccess {
        structure: Box<Localizable<ExpressionNode>>,
        field: Localizable<Box<str>>,
    },
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    VariableDeclaration {
        name: Localizable<Box<str>>,
        r#type: Localizable<Type>,
        value: Box<Localizable<ExpressionNode>>,
    },
    Return(Box<Localizable<ExpressionNode>>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Localizable<Box<str>>,
    pub arguments: Localizable<Box<[Localizable<ExpressionNode>]>>,
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
    String,
    Object(Box<str>),
}
