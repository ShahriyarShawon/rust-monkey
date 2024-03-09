use crate::token::*;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements
                .get(0)
                .expect("no statement there")
                .token_literal()
        } else {
            String::new()
        }
    }

    fn to_string(&self) -> String {
        let mut out_str = String::new();
        for s in &self.statements {
            out_str.push_str(s.to_string().as_str());
            // out_str.push('\n');
        }
        out_str
    }
}

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.to_string(),
            match &self.value {
                Some(v) => v.to_string(),
                None => "".to_string(),
            }
        );
    }
}

#[derive(Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        return self.value.to_string();
    }

}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let return_value = match &self.return_value {
            Some(rv) => rv.to_string(),
            None => "".to_string()
        };
        return format!("{} {};", 
                       self.token_literal(), 
                       return_value
                       );
    }
}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ExpressionStatement {
    token: Token,
    expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        match &self.expression {
            Some(e) => e.to_string(),
            None => "".to_string()
        }
    }
}

impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, Token};

    use super::{Identifier, LetStatement, Program, TokenType};

    #[test]
    fn test_string() {
        let program = Program{
            statements: vec![
                Box::new(LetStatement{
                    token: Token{token_type: TokenType::LET, literal: "let".to_string()},
                    name: Identifier{
                        token: Token{token_type: TokenType::IDENT, literal: "myVar".to_string()},
                        value: "myVar".to_string()
                    },
                    value: Some(Box::new(Identifier {
                        token: Token {token_type: TokenType::IDENT, literal: "anotherVar".to_string()},
                        value: "anotherVar".to_string()
                    })),
                },
            )]
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;".to_string(), "unexpected, got {}", program.to_string());
    }
}
