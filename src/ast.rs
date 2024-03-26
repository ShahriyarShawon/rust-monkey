use crate::token::*;

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(ls) => ls.token.literal.clone(),
            Statement::ReturnStatement(rs) => rs.token.literal.clone(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Statement::LetStatement(ls) => {
                return format!(
                    "{} {} = {};",
                    ls.token_literal(),
                    ls.name.to_string(),
                    match &ls.value {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    }
                )
            }
            Statement::ReturnStatement(rs) => {
                return format!(
                    "{} {};",
                    rs.token_literal(),
                    match &rs.return_value {
                        Some(rv) => rv.to_string(),
                        None => "".to_string(),
                    }
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(id) => id.token.literal.clone(),
            _ => panic!("token_literal"),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(id) => id.value.to_string(),
            _ => panic!("to_string"),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
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

#[derive(Default, Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
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

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let return_value = match &self.return_value {
            Some(rv) => rv.to_string(),
            None => "".to_string(),
        };
        return format!("{} {};", self.token_literal(), return_value);
    }
}

struct ExpressionStatement {
    token: Token,
    expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        match &self.expression {
            Some(e) => e.to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, Token};

    use super::{Expression, Identifier, LetStatement, Program, Statement, TokenType};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::LetStatement(LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Expression::Identifier(Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };

        assert_eq!(
            program.to_string(),
            "let myVar = anotherVar;".to_string(),
            "unexpected, got {}",
            program.to_string()
        );
    }
}
