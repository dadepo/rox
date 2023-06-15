use crate::token::TokenType::{
    BANG, BANGEQUAL, COMMA, DOT, EOF, EQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, IDENTIFIER,
    LEFTBRACE, LEFTPAREN, LESS, LESSEQUAL, MINUS, NUMBER, PLUS, RIGHTBRACE, RIGHTPAREN, SEMICOLON,
    SLASH, STAR, STRING,
};
use crate::token::{Token, TokenType, KEYWORDS};
use anyhow::{anyhow, Result};

use std::any::Any;
use std::str::FromStr;

pub fn run(line: String) -> Result<Vec<Token>> {
    let scanner = Scanner::new(line);
    scanner.scan_tokens()
}

pub fn error(line: u32, msg: &str) {
    println!("[line {}] Error: {}", line, msg)
}

#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            ..Scanner::default()
        }
    }
    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        loop {
            // done, at end, exist
            if self.is_at_end() {
                break;
            }
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(EOF, None)?;
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let current_char = self.get_current_and_advance_cursor();
        let _ = match current_char {
            '(' => self.add_token(LEFTPAREN, None),
            ')' => self.add_token(RIGHTPAREN, None),
            '{' => self.add_token(LEFTBRACE, None),
            '}' => self.add_token(RIGHTBRACE, None),
            ',' => self.add_token(COMMA, None),
            '.' => self.add_token(DOT, None),
            '-' => self.add_token(MINUS, None),
            '+' => self.add_token(PLUS, None),
            ';' => self.add_token(SEMICOLON, None),
            '*' => self.add_token(STAR, None),
            '!' => {
                if self.next_is('=') {
                    self.add_token(BANGEQUAL, None)
                } else {
                    self.add_token(BANG, None)
                }
            }
            '=' => {
                if self.next_is('=') {
                    self.add_token(EQUALEQUAL, None)
                } else {
                    self.add_token(EQUAL, None)
                }
            }
            '<' => {
                if self.next_is('=') {
                    self.add_token(LESSEQUAL, None)
                } else {
                    self.add_token(LESS, None)
                }
            }
            '>' => {
                if self.next_is('=') {
                    self.add_token(GREATEREQUAL, None)
                } else {
                    self.add_token(GREATER, None)
                }
            }
            '/' => {
                if self.next_is('/') {
                    // we have a comment, so keep advancing till you hit the new line
                    loop {
                        if self.peek() == '\n' && !self.is_at_end() {
                            self.get_current_and_advance_cursor();
                        }
                    }
                } else {
                    self.add_token(SLASH, None)
                }
            }
            ' ' | '\r' | '\t' => {
                // do nothing
                Ok(())
            }
            '\n' => {
                self.current += 1;
                Ok(())
            }
            '"' => {
                let value = self.extract_string()?;
                let _ = self.add_token(STRING, Some(Box::new(value)));
                Ok(())
            }
            _ => {
                if Self::is_digit(current_char) {
                    let value = self.extract_number()?;
                    let _ = self.add_token(NUMBER, Some(Box::new(value)));
                    Ok(())
                } else if Self::is_alpha(current_char) {
                    let value = self.extract_identifier()?;
                    match KEYWORDS.get(&value.as_ref()) {
                        Some(reserved_type) => {
                            self.add_token(reserved_type.to_owned(), None)?;
                        }
                        None => {
                            self.add_token(IDENTIFIER, None)?;
                        }
                    }
                    Ok(())
                } else {
                    error(self.line, "Unexpected character");
                    Ok(())
                }
            }
        };
        Ok(())
    }

    fn is_digit(input: char) -> bool {
        input.is_ascii_digit()
    }

    fn is_alpha(input: char) -> bool {
        input.is_ascii()
    }

    fn is_alpha_numeric(input: char) -> bool {
        input.is_ascii_alphanumeric()
    }

    fn extract_number(&mut self) -> Result<f64> {
        while Self::is_digit(self.peek()) {
            self.get_current_and_advance_cursor();
        }

        if self.peek() == '.' && Self::is_digit(self.double_peek()) {
            // this consumes the .
            self.get_current_and_advance_cursor();
            while Self::is_digit(self.peek()) {
                self.get_current_and_advance_cursor();
            }
        }

        let lexeme = &self.source.as_bytes()[self.start as usize..self.current as usize];
        let lexeme_str = std::str::from_utf8(lexeme)
            .map(|r| r.to_string())
            .map_err(|e| anyhow!(e))?;
        f64::from_str(&lexeme_str).map_err(|e| anyhow!(e))
    }

    fn extract_identifier(&mut self) -> Result<String> {
        while Self::is_alpha_numeric(self.peek()) {
            self.get_current_and_advance_cursor();
        }

        let lexeme = &self.source.as_bytes()[self.start as usize..self.current as usize];
        return std::str::from_utf8(lexeme)
            .map(|r| r.to_string())
            .map_err(|e| anyhow!(e));
    }

    fn extract_string(&mut self) -> Result<String> {
        loop {
            if self.peek() == '"' {
                // get't the last '"'
                self.get_current_and_advance_cursor();
                let lexeme =
                    &self.source.as_bytes()[(self.start + 1) as usize..(self.current - 1) as usize];
                return std::str::from_utf8(lexeme)
                    .map(|r| r.to_string())
                    .map_err(|e| anyhow!(e));
            }
            if self.is_at_end() {
                error(self.line, "Unterminated string");
                return Err(anyhow!("Unterminated string"));
            }

            self.line += 1;
            self.get_current_and_advance_cursor();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let next_char = self.source.as_bytes()[self.current as usize] as char;
            next_char
        }
    }

    fn double_peek(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            '\0'
        } else {
            let next_next_char = self.source.as_bytes()[(self.current + 1) as usize] as char;
            next_next_char
        }
    }

    fn next_is(&mut self, item: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let next_char = self.source.as_bytes()[self.current as usize] as char;
        if next_char != item {
            false
        } else {
            // increase current position since we will consume the matched item
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType, value: Option<Box<dyn Any>>) -> Result<()> {
        let lexeme = &self.source.as_bytes()[self.start as usize..self.current as usize];
        let lexeme = std::str::from_utf8(lexeme)?.to_string();
        let token = Token::new(token_type, lexeme, value, self.line);
        self.tokens.push(token);
        Ok(())
    }

    fn get_current_and_advance_cursor(&mut self) -> char {
        let item = self.source.as_bytes()[self.current as usize] as char;
        self.current += 1;
        item
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}
