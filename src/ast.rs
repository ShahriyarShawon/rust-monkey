use crate::token::*;
use std::any::Any;


pub trait Node: {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Eq, PartialEq)]
pub enum StatementType {
    LetStatement
}

pub trait Statement: Node {
    fn statement_type(&self) -> StatementType; 
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements.get(0).expect("no statement there").token_literal()
        } else {
            String::new()
        }
    }
}

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>
}

impl Statement for LetStatement {
    fn statement_type(&self) -> StatementType { StatementType::LetStatement }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
