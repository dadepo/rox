//
//  * filter     -> "list" filter
//  *
//  *
//  * expression  →          literal
//  *                      | unary
//  *                      | binary
//  *                      | grouping ;
//  *
//  * grouping →          "(" expression ")" ;
//  * binary   →          expression operator expression ;
//  * unary    →          ( "-" | "!" ) expression ;
//  * literal  →          NUMBER | STRING | "true" | "false" | "nil" ;
//  * operator →          "==" | "!=" | "<" | "<=" | ">"
//  */
//     public static void main(String[] args) {
//         Expr binary = new Expr.Binary(
//
//                 new Expr.Unary(
//                         new Token(TokenType.MINUS, "-", null, 1),
//                         new Expr.Literal(123)),
//
//                 new Token(TokenType.STAR, "*", null, 1),
//
//                 new Expr.Grouping(
//                         new Expr.Literal(45.67))
//         );
//
//
//         System.out.println(new AstPrinter().print(binary));
//
//     }

use std::any::Any;

use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Literal(Box<dyn Any>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>)
}
