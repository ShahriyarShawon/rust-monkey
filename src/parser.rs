use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

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

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        // let <identifier> = <expression>;
        let mut stmt = Box::new(LetStatement {
            token: self.cur_token.clone(),
            value: None,
            ..Default::default()
        });

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

        Some(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        // return <expression>;
        let stmt = Box::new(ReturnStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        });

        self.next_token();

        // TODO: skipping expressions until encounter semicolon;

        loop {
            if self.cur_token_is(TokenType::SEMICOLON) {
                break;
            } else {
                self.next_token();
            }
        }

        Some(stmt)
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
}

#[cfg(test)]
mod tests {
    use super::{LetStatement, Lexer, Node, Parser, ReturnStatement, Statement};
    use crate::lexer;

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

    fn test_let_statement(stmt: &Box<dyn Statement>, name: &str) -> bool {
        if stmt.token_literal() == "let" {
            println!("TokenLiteral not 'let', got {}", stmt.token_literal());
            return false;
        }

        let letstmt = match stmt.as_any().downcast_ref::<LetStatement>() {
            Some(statement) => statement,
            None => {
                print!("statement is not let statement");
                return false;
            }
        };

        if letstmt.name.value != name {
            println!(
                "let statement value not {}, instead got {}",
                name, letstmt.name.value
            );
            return false;
        }

        if letstmt.name.token_literal() != name {
            println!(
                "letstatement token literal not {}, instead got {}",
                name,
                letstmt.name.token_literal()
            );
            return false;
        }
        true
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
            let returnstmt = match stmt.as_any().downcast_ref::<ReturnStatement>() {
                Some(statement) => statement,
                None => {
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
}
