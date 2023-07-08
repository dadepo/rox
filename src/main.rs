use std::{env, fs, process};
use std::rc::Rc;

use rustyline::{DefaultEditor, Result};
use rustyline::error::ReadlineError;
use crate::environment::Environment;

use crate::parser::Parser;
use crate::scanner::run;
use crate::stmt::Stmt;
use crate::visitor::Interpreter;

mod expr;
mod parser;
mod predicate;
mod scanner;
mod token;
mod visitor;
mod stmt;
mod environment;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(1);
    }

    if args.len() == 1 {
        let file_content = fs::read_to_string(args.remove(0))?;
        let tokens = run(file_content).unwrap();
        let mut parser = Parser::new(tokens);
        let stmts: Vec<Rc<dyn Stmt>> = parser.parse().unwrap();
        let mut interpreter = Interpreter::new(Environment::new());
        println!("Evaluated: {:?}", interpreter.interpret(stmts));
        process::exit(1);
    }

    let mut rl = DefaultEditor::new()?;
    rl.load_history("history.txt").ok();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let tokens = run(line).unwrap();
                let mut parser = Parser::new(tokens);
                let stmts: Vec<Rc<dyn Stmt>> = parser.parse().unwrap();
                // let mut ast_printer = AstPrinter::new();
                // println!("Tokens: {:?}", ast_printer.print(Rc::clone(&stmts)));
                let mut interpreter = Interpreter::new(Environment::new());
                println!("Evaluated: {:?}", interpreter.interpret(stmts));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").ok();
    Ok(())
}
