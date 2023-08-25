use std::rc::Rc;
use std::{env, fs, process};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::run;
use crate::stmt::Stmt;

mod class;
mod environment;
mod expr;
mod functions;
mod interpreter;
mod parser;
mod predicate;
mod resolver;
mod scanner;
mod stmt;
mod token;
mod visitor;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 1 {
        println!("Usage: rox [script]");
        process::exit(1);
    }

    if args.len() == 1 {
        let file_content = fs::read_to_string(args.remove(0))?;
        let tokens = run(file_content).unwrap();
        let mut parser = Parser::new(tokens);
        let stmts: Vec<Rc<dyn Stmt>> = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let mut resolver = Resolver::new(&interpreter);
        resolver.resolve(stmts.clone()).unwrap();

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
                let mut interpreter = Interpreter::new();

                let mut resolver = Resolver::new(&interpreter);
                resolver.resolve(stmts.clone()).unwrap();

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
