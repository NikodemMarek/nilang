use std::collections::HashMap;

use super::{
    expressions::{Conditional, ExpressionNode, FunctionCall},
    Type,
};

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
