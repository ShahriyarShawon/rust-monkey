use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    char_list: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: '\0',
            char_list: Vec::new(),
        };

        l.read_char();
        l.char_list = l.input.chars().collect();

        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '+' => self.new_token(TokenType::PLUS, self.ch),
            '-' => self.new_token(TokenType::MINUS, self.ch),
            '*' => self.new_token(TokenType::ASTERISK, self.ch),
            '/' => self.new_token(TokenType::SLASH, self.ch),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token {
                        token_type: TokenType::EQ,
                        literal: "==".to_string(),
                    };
                } else {
                    self.new_token(TokenType::ASSIGN, self.ch)
                }
            }
            '>' => self.new_token(TokenType::GT, self.ch),
            '<' => self.new_token(TokenType::LT, self.ch),
            ',' => self.new_token(TokenType::COMMA, self.ch),
            ';' => self.new_token(TokenType::SEMICOLON, self.ch),
            '(' => self.new_token(TokenType::LPAREN, self.ch),
            ')' => self.new_token(TokenType::RPAREN, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token {
                        token_type: TokenType::NotEq,
                        literal: "!=".to_string(),
                    };
                } else {
                    self.new_token(TokenType::BANG, self.ch)
                }
            }
            '{' => self.new_token(TokenType::LBRACE, self.ch),
            '}' => self.new_token(TokenType::RBRACE, self.ch),
            '\0' => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },

            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    return Token {
                        literal: literal.clone(),
                        token_type: lookup_ident(literal),
                    };
                } else if self.ch.is_digit(10) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number(),
                    };
                } else {
                    return Token {
                        token_type: TokenType::EOF,
                        literal: self.ch.to_string(),
                    };
                }
            }
        };
        self.read_char();
        tok
    }

    pub fn new_token(&self, token_type: TokenType, ch: char) -> Token {
        let mut buffer = [0; 1];
        ch.encode_utf8(&mut buffer);
        let literal = std::str::from_utf8(&buffer).unwrap().to_owned();
        Token {
            token_type,
            literal,
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;

        loop {
            if self.ch.is_alphabetic() {
                self.read_char()
            } else {
                break;
            }
        }
        self.get_substring(pos, self.position)
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;

        loop {
            if self.ch.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }

        self.get_substring(pos, self.position)
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char()
            } else {
                break;
            }
        }
    }

    pub fn get_substring(&self, start: usize, end: usize) -> String {
        String::from_iter(self.char_list[start..end].iter())
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.char_list.get(self.read_position).unwrap().clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

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
            (TokenType::BANG, "!".to_string()),
            (TokenType::MINUS, "-".to_string()),
            (TokenType::SLASH, "/".to_string()),
            (TokenType::ASTERISK, "*".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::GT, ">".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::IF, "if".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::RETURN, "return".to_string()),
            (TokenType::TRUE, "true".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::ELSE, "else".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::RETURN, "return".to_string()),
            (TokenType::FALSE, "false".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::EQ, "==".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::NotEq, "!=".to_string()),
            (TokenType::INT, "9".to_string()),
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
