mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use repl::*;
use token::*;

fn main() {
    StartRepl();
}
