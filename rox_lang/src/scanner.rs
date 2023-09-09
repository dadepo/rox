use std::rc::Rc;
use crate::scanner::TokenType::{BANG, BANG_EQUAL, COMMA, DOT, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LEFT_BRACE, LEFT_PAREN, LESS, LESS_EQUAL, MINUS, NUMBER, PLUS, RIGHT_BRACE, RIGHT_PAREN, SEMICOLON, SLASH, STAR};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS,
    SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE,
    FOR, FUN, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE,
    ERROR, EOF,
}

pub struct Token {
    pub token_type: TokenType,
    pub token: Rc<String>,
    pub line: i8
}

pub struct Scanner {
    pub start_index: usize,
    pub current_index: usize,
    pub line: i8,
    pub code: Vec<char>
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Self {
            start_index: 0,
            current_index: 0,
            line: 1,
            code: code.chars().collect()
        }
    }

    pub fn scan_token(&mut self) -> Token {
        if self.is_at_end() {
            self.make_token(TokenType::EOF)
        } else {
            self.skip_white_spaces();
            self.start_index = self.current_index;
            let c: char = self.advance();
            if Scanner::is_digit(c) {
                return self.number();
            }
            if Scanner::is_alpha(c) {
                return self.identifier();
            }

            match c {
                '(' => self.make_token(LEFT_PAREN),
                ')' => self.make_token(RIGHT_PAREN),
                '{' => self.make_token(LEFT_BRACE),
                '}' => self.make_token(RIGHT_BRACE),
                ';' => self.make_token(SEMICOLON),
                ',' => self.make_token(COMMA),
                '.' => self.make_token(DOT),
                '-' => self.make_token(MINUS),
                '+' => self.make_token(PLUS),
                '/' => self.make_token(SLASH),
                '*' => self.make_token(STAR),
                '!' => {
                    let token_type = if self.advance_if('=') { BANG_EQUAL } else { BANG };
                    self.make_token(token_type)
                },
                '=' => {
                    let token_type = if self.advance_if('=') { EQUAL_EQUAL } else { EQUAL };
                    self.make_token(token_type)
                },
                '<' => {
                    let token_type = if self.advance_if('=') { LESS_EQUAL } else { LESS };
                    self.make_token(token_type)
                },
                '>' => {
                    let token_type = if self.advance_if('=') { GREATER_EQUAL } else { GREATER };
                    self.make_token(token_type)
                },
                '"' => {
                    self.string()
                },
                _ => self.error_token(&format!("Unknown character: {c}")),
            }

        }
    }

    fn skip_white_spaces(&mut self) {
        loop {
            let c = self.peek().expect("peek error skip_white_spaces");
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                },
                '/' => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        continue
                    }
                },
                _ => {
                    break
                }
            }
        }
    }

    fn advance_if(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            match self.code.get(self.current_index).copied() {
                Some(current_char) => {
                    if current_char == expected {
                        self.current_index += 1;
                        true
                    } else {
                        false
                    }
                },
                None => false
            }
        }
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_';
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.code.get(0usize).copied() {
            Some('a') => self.check_keyword(1, 2, "and", TokenType::AND),
            Some('c') => self.check_keyword(1, 4, "lass", TokenType::CLASS),
            Some('e') => self.check_keyword(1, 3, "lse", TokenType::ELSE),
            Some('f') => {
                return if self.current_index - self.start_index > 1 {
                    return match self.code.get(1usize).copied() {
                        Some('a') => self.check_keyword(2, 3, "lse", TokenType::ELSE),
                        Some('o') => self.check_keyword(2, 1, "r", TokenType::FOR),
                        Some('u') => self.check_keyword(2, 1, "n", TokenType::FUN),
                        _ => panic!("TODO")
                    };
                } else {
                    panic!("TODO")
                };
            },
            Some('i') => self.check_keyword(1, 1, "f", TokenType::IF),
            Some('n') => self.check_keyword(1, 2, "il", TokenType::NIL),
            Some('o') => self.check_keyword(1, 1, "r", TokenType::OR),
            Some('p') => self.check_keyword(1, 4, "rint", TokenType::PRINT),
            Some('r') => self.check_keyword(1, 5, "eturn", TokenType::RETURN),
            Some('s') => self.check_keyword(1, 4, "uper", TokenType::SUPER),
            Some('t') => {
                return if self.current_index - self.start_index > 1 {
                    return match self.code.get(1usize).copied() {
                        Some('h') => self.check_keyword(2, 2, "is", TokenType::THIS),
                        Some('r') => self.check_keyword(2, 2, "ue", TokenType::TRUE),
                        _ => panic!("TODO")
                    }
                } else {
                    panic!("TODO")
                };
            },
            Some('v') => self.check_keyword(1, 2, "ar", TokenType::VAR),
            Some('w') => self.check_keyword(1, 4, "hile", TokenType::WHILE),
            _ => TokenType::IDENTIFIER
        }
    }

    fn check_keyword(&self, start: u8, length: u8, rest: &str, token_type: TokenType) -> TokenType {
        let found = &self.code[start as usize..((start + length) as usize)];
        let rest: Vec<char> = rest.chars().collect();
        if found == &rest[..] {
            token_type
        } else {
            TokenType::IDENTIFIER
        }
    }

    fn current_char(&self, index: usize) -> Option<char> {
       self.code.get(index).copied()
    }

    fn peek(&self) -> Option<char> {
        self.code.get(self.current_index).copied()
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        } else {
            self.code.get(self.current_index + 1).copied()
        }
    }

    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.code.get(self.current_index - 1).copied().expect("ERROR advancing")
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.code.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let value = &self.code[self.start_index..self.current_index];
        let value: String = value.iter().collect();
        Token {
            token_type,
            token: Rc::new(value),
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        return self.make_token(TokenType::STRING)
    }

    fn number(&mut self) -> Token {
        while !self.is_at_end() && Scanner::is_digit(self.peek().expect("peek in number()")) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next().expect("peek error is_digit")) {
            self.advance();
            while Scanner::is_digit(self.peek().expect("peek error is_digit")) {
                self.advance();
            }
        }

        self.make_token(NUMBER)
    }

    fn identifier(&mut self) -> Token {
        while !self.is_at_end() && (Scanner::is_alpha(self.peek().expect("peek error is_alpha")) || Scanner::is_digit(self.peek().expect("TODO"))) {
            self.advance();
        }
        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    fn error_token(&self, error_message: &str) -> Token {
        Token {
            token_type: TokenType::ERROR,
            token: Rc::new(error_message.to_string()),
            line: self.line,
        }
    }
}
