use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

#[allow(dead_code)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: "\0".to_string(),
            },
            peek_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: "\0".to_string(),
            },
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();
        p
    }

    #[allow(dead_code)]
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_error(&mut self, token: &TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        loop {
            if self.cur_token.token_type == TokenType::EOF {
                break;
            }
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => program.statements.push(s),
                None => {}
            }
            self.next_token()
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        // let <identifier> = <expression>;
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            value: None,
            ..Default::default()
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: skipping expression suntil we encounter semicolon
        loop {
            if self.cur_token_is(TokenType::SEMICOLON) {
                self.next_token();
            } else {
                break;
            }
        }

        Some(Statement::LetStatement(stmt))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        // return <expression>;
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        };

        self.next_token();

        // TODO: skipping expressions until encounter semicolon;

        loop {
            if self.cur_token_is(TokenType::SEMICOLON) {
                break;
            } else {
                self.next_token();
            }
        }

        Some(Statement::ReturnStatement(stmt))
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        dbg!("parse_expression_statement");
        let stmt = Statement::ExpressionStatement(ExpressionStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        });

        if let Statement::ExpressionStatement(mut es) = stmt.clone() {
            es.expression = self.parse_expression(Precedence::LOWEST);

            if self.peek_token_is(&TokenType::SEMICOLON) {
                self.next_token();
            }

            return Some(Statement::ExpressionStatement(es));
        }
        None
    }

    pub fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression> {
        dbg!("parse_expression");
        let exp = self.prefix_parse_fn(&self.cur_token.token_type);
        dbg!(&exp);

        exp
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }

    fn prefix_parse_fn(&self, token_type: &TokenType) -> Option<Expression> {
        dbg!("prefix_parse_fn");
        match token_type {
            TokenType::IDENT => Some(self.parse_identifier()),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn infix_parse_fn(&self, _token_type: TokenType) {}

    fn parse_identifier(&self) -> Expression {
        dbg!("parse_identifier");
        return Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Node, Parser,Statement};
    use crate::{
        ast::Expression,
        lexer,
    };

    fn check_parser_errors(p: &Parser) {
        let error_count = p.errors.len();
        if error_count == 0 {
            return;
        }

        println!("parser has {} errors", error_count);
        for msg in p.errors() {
            println!("parser error: {}", msg);
        }
        assert!(false);
    }
    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foo = 838383;
"#;

        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            println!(
                "program statements does not container 3 statements, got {}",
                program.statements.len()
            );
        }

        let tests = vec!["x", "y", "foobar"];

        for (i, item) in tests.iter().enumerate() {
            let stmt = program.statements.get(i).unwrap();
            if !test_let_statement(stmt, item) {
                return;
            }
        }
    }

    fn test_let_statement(stmt: &Statement, name: &str) -> bool {
        if stmt.token_literal() == "let" {
            println!("TokenLiteral not 'let', got {}", stmt.token_literal());
            return false;
        }

        if let Statement::LetStatement(ls) = stmt {
            if ls.name.value != name {
                println!(
                    "LetStatement.Name.Value not {}, got {}",
                    name, ls.name.value
                );
                return false;
            }
            if ls.name.token_literal() != name {
                println!(
                    "LetStatement.Name.TokenLiteral not {} got {}",
                    name,
                    ls.name.token_literal()
                );
                return false;
            }
            return true;
        } else {
            println!("statement is not LetStatement");
            return false;
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            3,
            "program statements does not contain 3 statements, got {}",
            program.statements.len()
        );

        for stmt in program.statements {
            let returnstmt = match stmt {
                Statement::ReturnStatement(rs) => rs,
                _ => {
                    assert!(false, "statement is not a return statement");
                    continue;
                }
            };
            if returnstmt.token_literal() != "return" {
                assert!(
                    false,
                    "return statement literal not 'return', got {}",
                    returnstmt.token_literal()
                );
                continue;
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            1,
            program.statements.len(),
            "Program has not enough statements, got {}",
            program.statements.len()
        );


        if let Statement::ExpressionStatement(mut es) = program.statements.get(0).unwrap().clone() {
            dbg!(&es);
            #[allow(irrefutable_let_patterns)]
            if let Expression::Identifier(id) = es.expression.as_mut().unwrap() {
                assert_eq!(
                    &id.value, "foobar",
                    "ident value not {}, got {}",
                    "foobar", &id.value
                );
                assert_eq!(
                    &id.token_literal(),
                    "foobar",
                    "ident token literal not {}, got {}",
                    "foobar",
                    &id.token_literal()
                );
            } else {
                assert!(false, "exp not Identifier");
            }
        } else {
            assert!(false, "program statements [0] is not ExpressionStatement");
        }
    }
}
