use std::{collections::HashMap, fmt::Debug};

use crate::Localizable as L;

pub type Str = Box<str>;

pub type TypedIdentifier = (L<Str>, L<Type>);
pub type Parameters = Box<[TypedIdentifier]>;

pub type FunctionBody = Box<[L<StatementNode>]>;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: L<Str>,
    pub parameters: L<Parameters>,
    pub return_type: L<Type>,
    pub body: L<FunctionBody>,
}

pub type StructureFields = HashMap<L<Str>, L<Type>>;

#[derive(Debug, Clone, PartialEq)]
pub struct StructureDeclaration {
    pub name: L<Str>,
    pub fields: L<StructureFields>,
}

pub type ObjectFields = HashMap<L<Str>, L<ExpressionNode>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Number(f64),
    Char(char),
    String(Str),
    Object {
        r#type: L<Type>,
        fields: L<ObjectFields>,
    },
    Operation {
        operator: L<Operator>,
        a: Box<L<ExpressionNode>>,
        b: Box<L<ExpressionNode>>,
    },
    VariableReference(Str),
    FieldAccess {
        structure: Box<L<ExpressionNode>>,
        field: L<Str>,
    },
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    VariableDeclaration {
        name: L<Str>,
        r#type: L<Type>,
        value: Box<L<ExpressionNode>>,
    },
    Return(Box<L<ExpressionNode>>),
    FunctionCall(FunctionCall),
}

pub type Arguments = Box<[L<ExpressionNode>]>;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: L<Str>,
    pub arguments: L<Arguments>,
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
    Object(Str),
}
