use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::TokenType::{
    BANG, BANGEQUAL, EOF, EQUALEQUAL, FALSE, GREATER, GREATEREQUAL, LEFTPAREN, LESS, LESSEQUAL,
    MINUS, NIL, NUMBER, PLUS, RIGHTPAREN, SLASH, STAR, STRING, TRUE,
};
use crate::token::{DataType, Token, TokenType};
use anyhow::anyhow;
use anyhow::Result;
use std::rc::Rc;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
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
        Parser { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Rc<dyn Expr>> {
        self.expression()
    }
    // expression → equality
    pub fn expression(&mut self) -> Result<Rc<dyn Expr>> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )
    pub fn equality(&mut self) -> Result<Rc<dyn Expr>> {
        let mut left = self.comparison()?;

        while self.match_token(vec![BANGEQUAL, EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            left = Rc::new(BinaryExpr {
                left,
                operator,
                right,
            });
        }

        Ok(left)
    }

    pub fn comparison(&mut self) -> Result<Rc<dyn Expr>> {
        let mut left = self.term()?;
        while self.match_token(vec![GREATER, GREATEREQUAL, LESS, LESSEQUAL]) {
            let operator = self.previous();
            let right = self.term()?;
            left = Rc::new(BinaryExpr {
                left,
                operator,
                right,
            });
        }
        Ok(left)
    }

    pub fn term(&mut self) -> Result<Rc<dyn Expr>> {
        let mut left = self.factor()?;
        while self.match_token(vec![MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            left = Rc::new(BinaryExpr {
                left,
                operator,
                right,
            });
        }
        Ok(left)
    }

    pub fn factor(&mut self) -> Result<Rc<dyn Expr>> {
        let mut left = self.unary()?;

        while self.match_token(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            left = Rc::new(BinaryExpr {
                left,
                operator,
                right,
            });
        }

        Ok(left)
    }

    pub fn unary(&mut self) -> Result<Rc<dyn Expr>> {
        if self.match_token(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Rc::new(UnaryExpr { operator, right }));
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Result<Rc<dyn Expr>> {
        if self.match_token(vec![TRUE]) {
            return Ok(Rc::new(LiteralExpr {
                value: Some(DataType::Bool(true)),
            }));
        }
        if self.match_token(vec![FALSE]) {
            return Ok(Rc::new(LiteralExpr {
                value: Some(DataType::Bool(false)),
            }));
        }
        if self.match_token(vec![NIL]) {
            return Ok(Rc::new(LiteralExpr {
                value: Some(DataType::Nil),
            }));
        }
        if self.match_token(vec![NUMBER, STRING]) {
            return Ok(Rc::new(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.match_token(vec![LEFTPAREN]) {
            let expression = self.expression()?;
            if self.consume(RIGHTPAREN).is_ok() {
                return Ok(Rc::new(GroupingExpr { expression }));
            }
        }

        Err(anyhow!("Unknown token"))
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
                return true;
            }
        }
        false
    }

    fn get_current_and_advance_cursor(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            match self.peek() {
                Some(next) => next.token_type == token_type,
                None => false,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(end) => end.token_type == EOF,
            None => true,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current as usize)
    }

    fn previous(&mut self) -> Token {
        self.tokens
            .get((self.current - 1) as usize)
            .unwrap()
            .to_owned()
    }
}
