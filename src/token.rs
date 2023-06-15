use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::AND);
        map.insert("class", TokenType::CLASS);
        map.insert("else", TokenType::ELSE);
        map.insert("false", TokenType::FALSE);
        map.insert("for", TokenType::FOR);
        map.insert("fun", TokenType::FUN);
        map.insert("if", TokenType::IF);
        map.insert("nil", TokenType::NIL);
        map.insert("or", TokenType::OR);
        map.insert("print", TokenType::PRINT);
        map.insert("return", TokenType::RETURN);
        map.insert("super", TokenType::SUPER);
        map.insert("this", TokenType::THIS);
        map.insert("true", TokenType::TRUE);
        map.insert("var", TokenType::VAR);
        map.insert("while", TokenType::WHILE);
        map
    };
}

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum TokenType {
    // Single character token
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character token
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals
    // variable name?
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords (can I see this as reserved identifiers?)
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Option<Box<dyn Any>>,
    pub line: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        value: Option<Box<dyn Any>>,
        line: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            value,
            line,
        }
    }
}
