use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum TokenType {
    #[default]
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
    NotEq,

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
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
    ]);

    keywords.get(literal).cloned()
}

pub fn lookup_ident(ident: String) -> TokenType {
    let tok = match keywords(ident.as_str()) {
        Some(tt) => tt.clone(),
        None => TokenType::IDENT,
    };
    return tok;
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
