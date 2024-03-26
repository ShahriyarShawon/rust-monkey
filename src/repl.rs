use std::io::{self, Write};

use crate::{lexer::*, parser::*, TokenType};

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
            }
            Err(err) => {
                panic!("somethings gone wrong {:?}", err)
            }
        }
        buffer = buffer.trim().to_string();
        println!("{}", buffer);

        let mut lexer = Lexer::new(buffer.as_str());
        // loop {
        //     let tok = lexer.next_token();
        //     if tok.token_type != TokenType::EOF {
        //         println!("{:?}", tok);
        //     } else {
        //         println!("{:?}", tok);
        //         break;
        //     }
        // }
        println!("created lexer");
        let mut parser = Parser::new(lexer);
        let p = parser.parse_program();
        dbg!(p);
    }
}
