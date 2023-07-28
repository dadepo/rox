use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;

use crate::expr::{AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr, VarExpr};
use crate::functions::Kind;
use crate::scanner::error;
use crate::stmt::{BlockStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt};
use crate::token::TokenType::{AND, BANG, BANGEQUAL, CLASS, COMMA, ELSE, EOF, EQUAL, EQUALEQUAL, FALSE, FOR, FUN, GREATER, GREATEREQUAL, IDENTIFIER, IF, LEFTBRACE, LEFTPAREN, LESS, LESSEQUAL, MINUS, NIL, NUMBER, OR, PLUS, PRINT, RETURN, RIGHTBRACE, RIGHTPAREN, SEMICOLON, SLASH, STAR, STRING, TRUE, VAR, WHILE};
use crate::token::{DataType, Token, TokenType};

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
        let result = if self.match_token(vec![FUN]) {
          self.function(Kind::Function)
        } else if self.match_token(vec![VAR]) {
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

    fn function(&mut self, _kind: Kind) -> Result<Rc<dyn Stmt>> {
        let name = self.consume(IDENTIFIER)?;
        self.consume(LEFTPAREN)?;
        let mut params = vec![];
        if !self.check(RIGHTPAREN) {
            loop {

                if params.len() >= 255 {
                    dbg!("Can't have more than 255 parameters.");
                }
                params.push(self.consume(IDENTIFIER)?);
                if !self.match_token(vec![COMMA]) {
                    break;
                }
            }
        }
        self.consume(RIGHTPAREN)?;
        self.consume(LEFTBRACE)?;
        let body = self.block()?;

        Ok(Rc::new(FunctionStmt {
            name,
            params,
            body
        }))
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
        if self.match_token(vec![FOR]) {
            self.for_statement()
        } else if self.match_token(vec![IF]) {
            self.if_statement()
        } else if self.match_token(vec![PRINT]) {
            self.print_statement()
        } else if self.match_token(vec![WHILE]) {
            self.while_statement()
        } else if self.match_token(vec![LEFTBRACE]) {
            Ok(Rc::new(BlockStmt {
                statements: self.block()?,
            }))
        } else {
            self.expression_statement()
        }
    }

    pub fn for_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        self.consume(LEFTPAREN)?;
        let init = if self.match_token(vec![SEMICOLON]) {
            None
        } else if self.match_token(vec![VAR]) {
            Some(self.var_declaration()?)
        } else {
           Some(self.expression_statement()?)
        };

        let mut condition = if !self.check(SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(SEMICOLON)?;

        let increment = if !self.check(RIGHTPAREN) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(RIGHTPAREN)?;

        let mut body = self.statement()?;

        if increment.is_some() {
            body = Rc::new(BlockStmt { statements: vec![body, Rc::new(ExprStmt { expression: increment.unwrap() })] })
        }

        if condition.is_none() {
            condition = Some(Rc::new(LiteralExpr { value: Some(DataType::Bool(true)) }))
        };

        body = Rc::new(WhileStmt {
            condition: condition.unwrap(),
            body,
        });

        if init.is_some() {
            body = Rc::new(BlockStmt { statements: vec![init.unwrap(), body] })
        }

        Ok(body)
    }

    pub fn while_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        self.consume(LEFTPAREN)?;
        let condition = self.expression()?;
        self.consume(RIGHTPAREN)?;
        let body = self.statement()?;
        Ok(Rc::new(WhileStmt { condition, body }))
    }

    pub fn if_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        self.consume(LEFTPAREN)?;
        let condition = self.expression()?;
        self.consume(RIGHTPAREN)?;

        let then_branch = self.statement()?;
        let else_branch: Option<Rc<dyn Stmt>> = if self.match_token(vec![ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Rc::new(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
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
        Ok(Rc::new(PrintStmt { expression: expr }))
    }

    pub fn expression_statement(&mut self) -> Result<Rc<dyn Stmt>> {
        let expr = self.expression()?;
        self.consume(SEMICOLON)?;
        Ok(Rc::new(ExprStmt { expression: expr }))
    }

    // expression → equality
    pub fn expression(&mut self) -> Result<Rc<dyn Expr>> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Result<Rc<dyn Expr>> {
        let expr = self.or()?;
        if self.match_token(vec![EQUAL]) {
            let _ = self.previous();
            let value = self.assignment()?;

            if expr.as_any().downcast_ref::<VarExpr>().is_some() {
                let var_name = expr
                    .as_any()
                    .downcast_ref::<VarExpr>()
                    .unwrap()
                    .var_name
                    .clone();
                return Ok(Rc::new(AssignExpr {
                    var_name,
                    var_value: Some(value),
                }));
            } else {
                dbg!("error");
            }
        }

        Ok(expr)
    }

    pub fn or(&mut self) -> Result<Rc<dyn Expr>> {
        let mut expr = self.and()?;
        while self.match_token(vec![OR]) {
            let operator: Token = self.previous();
            let right = self.and()?;
            expr = Rc::new(LogicalExpr {
                left: expr,
                operator,
                right
            });
        }
        Ok(expr)
    }

    pub fn and(&mut self) -> Result<Rc<dyn Expr>> {
        let mut expr = self.equality()?;
        while self.match_token(vec![AND]) {
            let operator: Token = self.previous();
            let right = self.equality()?;
            expr = Rc::new(LogicalExpr {
                left: expr,
                operator,
                right
            });
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

        self.call()
    }

    pub fn call(&mut self) -> Result<Rc<dyn Expr>> {
        let mut expr = self.primary()?;
        while true {
            if self.match_token(vec![LEFTPAREN]) {
                expr = self.finish_call(&expr)?;
            } else {
                break
            }
        }

        Ok(expr)
    }

    pub fn finish_call(&mut self, callee: &Rc<dyn Expr>) -> Result<Rc<dyn Expr>> {
        let mut arguments = vec![];
        if !self.check(RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    dbg!("Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self.match_token(vec![COMMA]) {
                    break
                }
            }
        }

        let paren = self.consume(RIGHTPAREN)?;

        Ok(Rc::new(CallExpr {
            callee: Rc::clone(callee),
            paren,
            arguments,
        }))
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

        if self.match_token(vec![IDENTIFIER]) {
            return Ok(Rc::new(VarExpr {
                var_name: self.previous(),
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
            // TODO accept the error message
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
                Some(next) => {
                    next.token_type == token_type
                },
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
                }
                _ => {
                    self.get_current_and_advance_cursor();
                }
            }
        }
        Ok(())
    }
}
