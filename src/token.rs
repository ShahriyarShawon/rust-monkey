use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOTEq,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

fn keywords(literal: &str) -> Option<TokenType> {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
    ]);

    keywords.get(literal).cloned()
}

pub fn lookup_ident(ident: String) -> TokenType {
    let tok = match keywords(ident.as_str()) {
        Some(tt) => tt.clone(),
        None => TokenType::IDENT
    };
    return tok
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
