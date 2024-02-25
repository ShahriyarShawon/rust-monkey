use crate::token::*;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => self.new_token(TokenType::ASSIGN, self.ch),
            ';' => self.new_token(TokenType::SEMICOLON, self.ch),
            '(' => self.new_token(TokenType::LPAREN, self.ch),
            ')' => self.new_token(TokenType::RPAREN, self.ch),
            ',' => self.new_token(TokenType::COMMA, self.ch),
            '+' => self.new_token(TokenType::PLUS, self.ch),
            '{' => self.new_token(TokenType::LBRACE, self.ch),
            '}' => self.new_token(TokenType::RBRACE, self.ch),


            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    Token {
                        literal: literal.clone(),
                        token_type: lookup_ident(literal)
                    }
                }else {
                    Token {
                        token_type: TokenType::EOF,
                        literal: "".to_string(),
                    }
                }

            },
        };
        self.read_char();
        tok
    }

    fn new_token(&self, token_type: TokenType, ch: char) -> Token {
        let mut buffer = [0; 1];
        ch.encode_utf8(&mut buffer);
        let literal = std::str::from_utf8(&buffer).unwrap().to_owned();
        Token {
            token_type,
            literal,
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        loop {
            if self.ch.is_alphabetic() {
                self.read_char()
            }
            else {
                break;
            }
        }
        let chars: Vec<char> = self.input.chars().collect();
        String::from_iter(chars[pos..self.position].iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    #[test]
    fn TestNextToken() {
        struct Test {
            expectedType: TokenType,
            expectedLiteral: String,
        }

        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten)"#;

        let tests: Vec<(TokenType, String)> = vec![
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "five".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "ten".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "add".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::FUNCTION, "fn".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::IDENT, "x".to_string()),
            (TokenType::COMMA, ",".to_string()),
            (TokenType::IDENT, "y".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::IDENT, "x".to_string()),
            (TokenType::PLUS, "+".to_string()),
            (TokenType::IDENT, "y".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "result".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::IDENT, "add".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::IDENT, "five".to_string()),
            (TokenType::COMMA, ",".to_string()),
            (TokenType::IDENT, "ten".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::EOF, "".to_string()),
        ];

        let mut l = Lexer::new(input);

        for tt in tests {
            let tok = l.next_token();

            let expected_type = tt.0;
            let expected_literal = tt.1;

            println!("Token: {:?}", tok);
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
