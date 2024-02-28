use std::io::{self, Write};

use crate::{lexer::*, TokenType};


const PROMPT: &str = ">> ";


pub fn StartRepl() {
    let mut buffer = String::new();

    loop {
        io::stdout().write_all(PROMPT.as_bytes());
        io::stdout().flush();
        buffer.clear();

        match io::stdin().read_line(&mut buffer) {
            Ok(s) => {
                if s == 0 {
                    break;
                }
            },
            Err(err) => {panic!("somethings gone wrong")}
        }
        buffer = buffer.trim().to_string();
        println!("{}", buffer);
    
        
        let mut lexer = Lexer::new(buffer.as_str());

        loop {
            let tok = lexer.next_token();
            if tok.token_type != TokenType::EOF {
                println!("{:?}", tok);
            } else {
                break;
            }
        }

    }
}
