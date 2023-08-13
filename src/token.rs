use std::collections::HashMap;
use std::fmt::Display;

use crate::class::{LoxClass, LoxInstance};
use crate::functions::{LoxFunction, LoxNative};
use lazy_static::lazy_static;

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

#[derive(Clone, Debug, PartialEq, Copy)]
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<DataType>,
    pub line: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<DataType>,
        line: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Function(LoxFunction),
    NativeFunction(LoxNative),
    Class(LoxClass),
    Instance(LoxInstance),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String(s) => write!(f, "{s}"),
            DataType::Number(n) => write!(f, "{n}"),
            DataType::Bool(b) => write!(f, "{b}"),
            DataType::Nil => write!(f, "NIL"),
            DataType::Function(func) => write!(f, "{func}"),
            DataType::NativeFunction(func) => write!(f, "{func}"),
            DataType::Class(class) => write!(f, "{class:?}"),
            DataType::Instance(instance) => write!(f, "{instance:?}"),
        }
    }
}
