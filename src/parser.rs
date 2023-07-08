use crate::expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VarExpr};
use crate::token::TokenType::{BANG, BANGEQUAL, EOF, EQUALEQUAL, FALSE, GREATER, GREATEREQUAL, LEFTPAREN, LESS, LESSEQUAL, MINUS, NIL, NUMBER, PLUS, PRINT, RIGHTPAREN, SEMICOLON, SLASH, STAR, STRING, TRUE, CLASS, FUN, VAR, FOR, IF, WHILE, RETURN, IDENTIFIER, EQUAL, LEFTBRACE, RIGHTBRACE};
use crate::token::{DataType, Token, TokenType};
use anyhow::anyhow;
use anyhow::Result;
use std::rc::Rc;
use crate::scanner::error;
use crate::stmt::{BlockStmt, ExprStmt, PrintStmt, Stmt, VarStmt};

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

    pub fn parse(&mut self) -> Result<Vec<Rc<dyn Stmt>>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        Ok(statements)
    }

    pub fn declaration(&mut self) -> Result<Rc<dyn Stmt>> {
        let result = if self.match_token(vec![VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(res) => Ok(res),
            Err(err) => {
                self.synchronise()?;
                Err(anyhow!(err))
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Rc<dyn Stmt>> {
        let var_name: Token = self.consume(IDENTIFIER)?;

        let var_value = if self.match_token(vec![EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(SEMICOLON)?;

        Ok(Rc::new(VarStmt {
            var_name,
            var_value,
        }))
    }

    pub fn statement(&mut self) -> Result<Rc<dyn Stmt>> {
        if self.match_token(vec![PRINT]) {
            self.print_statement()
        } else if self.match_token(vec![LEFTBRACE]) {
            Ok(Rc::new(BlockStmt {
                statements: self.block()?
            }))
        } else {
            self.expression_statement()
        }
    }

    pub fn block(&mut self) -> Result<Vec<Rc<dyn Stmt>>> {
        let mut statements = vec![];
        while !self.check(RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(RIGHTBRACE)?;
        Ok(statements)
    }

    pub fn print_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        let expr = self.expression()?;
        self.consume(SEMICOLON)?;
        Ok(Rc::new(PrintStmt {
            expression: expr,
        }))
    }

    pub fn expression_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        let expr = self.expression()?;
        self.consume(SEMICOLON)?;
        Ok(Rc::new(ExprStmt {
            expression: expr,
        }))
    }

    // expression → equality
    pub fn expression(&mut self) -> Result<Rc<dyn Expr>> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Result<Rc<dyn Expr>> {
        let expr = self.equality()?;
        if self.match_token(vec![EQUAL]) {
            let _ = self.previous();
            let value = self.assignment()?;

            if expr.as_any().downcast_ref::<VarExpr>().is_some() {
                let var_name = expr.as_any().downcast_ref::<VarExpr>().unwrap().var_name.clone();
                return Ok(Rc::new(AssignExpr {
                    var_name,
                    var_value: Some(value)
                }))
            } else {
                dbg!("error");
            }
        }

        Ok(expr)
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

        if self.match_token(vec![IDENTIFIER]) {
            return Ok(Rc::new(
                VarExpr { var_name: self.previous() }
            ));
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

    fn synchronise(&mut self) -> Result<()> {
        self.get_current_and_advance_cursor();
        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                break;
            }

            match self.peek().ok_or(anyhow!("can't peek"))?.token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => {
                    break;
                },
                _ => {
                    self.get_current_and_advance_cursor();
                }
            }
        }
        Ok(())
    }
}
