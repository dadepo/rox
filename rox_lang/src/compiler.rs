use crate::scanner::{Scanner, Token, TokenType};

pub fn compile(source: &str) -> () {
    let mut scanner = Scanner::new(source.to_string());
    let mut line = -1;
    loop {
         let token: Token = scanner.scan_token();
         if token.line != line {
             print!("{:4} ", token.line);
             line = token.line;
         } else {
             print!("   | ");
         }
        println!("{:2} '{}'", token.token_type, token.token);

        if token.token_type == TokenType::EOF {
            break;
        }
    }

}