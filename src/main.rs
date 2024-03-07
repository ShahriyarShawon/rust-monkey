mod lexer;
mod token;
mod repl;
mod ast;
mod parser;

use token::*;
use repl::*;

fn main() {
   StartRepl(); 
}
