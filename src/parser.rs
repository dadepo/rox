use anyhow::anyhow;
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::token::TokenType::{BANG, BANGEQUAL, EOF, EQUALEQUAL, FALSE, GREATER, GREATEREQUAL, LEFTPAREN, LESS, LESSEQUAL, MINUS, NIL, NUMBER, PLUS, RIGHTPAREN, SLASH, STAR, STRING, TRUE};

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: u32
}

/**
 * expression → equality ;
 * equality → comparison ( ( "!=" | "==" ) comparison ) ;
 * comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term → factor ( ( "-" | "+" ) factor )* ;
 * factor → unary ( ( "/" | "*" ) unary )* ;
 * unary → ( "!" | "-" ) unary
 * | primary ;
 * primary → NUMBER | STRING | "true" | "false" | "nil"
 * | "(" expression ")" ;
 */

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
    // expression → equality
    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )
    pub fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(vec![BANGEQUAL, EQUALEQUAL]) {
            let previous = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(
                Box::new(expr),
                previous.clone(),
                Box::new(right)
            )

        }

        expr
    }

    pub fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(vec![GREATER, GREATEREQUAL, LESS, LESSEQUAL]) {
            let previous = self.previous();
            let right = self.term();
            expr = Expr::Binary(
                Box::new(expr),
                previous.clone(),
                Box::new(right)
            );
        }
        expr
    }

    pub fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token(vec![MINUS, PLUS]) {
            let previous = self.previous();
            let right = self.term();
            expr = Expr::Binary(
                Box::new(expr),
                previous.clone(),
                Box::new(right)
            );
        }
        expr
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![SLASH, STAR]) {
            let previous = self.previous();
            let right = self.term();
            expr = Expr::Binary(
                Box::new(expr),
                previous.clone(),
                Box::new(right)
            );
        }

        expr
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_token(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.term();
           return  Expr::Unary(operator.clone(), Box::new(right))
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Expr {
        if self.match_token(vec![TRUE]) {
            return Expr::Literal(Box::new(true))
        }
        if self.match_token(vec![FALSE]) {
            return Expr::Literal(Box::new(true))
        }
        if self.match_token(vec![NIL]) {
            return Expr::Literal(Box::new(None::<String>))
        }
        if self.match_token(vec![NUMBER, STRING]) {
            return match self.previous().literal {
                Some(previous) => Expr::Literal(Box::new(previous)),
                None => Expr::Literal(Box::new(None::<String>))
            }
        }

        if self.match_token(vec![LEFTPAREN]) {
            let expr = self.expression();
            if self.consume(RIGHTPAREN).is_ok() {
                return Expr::Grouping(Box::new(expr))
            }
        }

        Expr::Literal(Box::new(None::<String>))
    }

    fn consume(&mut self, token_type: TokenType) -> anyhow::Result<Token> {
        if self.check(token_type) {
            Ok(self.get_current_and_advance_cursor())
        } else {
            Err(anyhow!("error"))
        }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token in token_types {
            if self.check(token) {
                self.get_current_and_advance_cursor();
                return true
            }
        }
        false
    }

    fn get_current_and_advance_cursor(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            match self.peek() {
                Some(next) => next.token_type == token_type,
                None => false
            }
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(end) => end.token_type == EOF,
            None => true
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current  as usize)
    }

    fn previous(&mut self) -> Token {
        self.tokens.remove((self.current - 1) as usize)
    }
}


