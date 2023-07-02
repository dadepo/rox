mod expr;
mod parser;
mod predicate;
mod scanner;
mod token;
mod visitor;

use crate::parser::Parser;
use crate::scanner::run;
use crate::visitor::{AstPrinter, Interpreter};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::{env, process};
use std::rc::Rc;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(1);
    }

    if args.len() == 1 {
        println!("Running from file not supported yet");
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
                let ast = parser.parse().unwrap();
                let mut ast_printer = AstPrinter::new();

                let mut interpreter = Interpreter::new();
                println!("Tokens: {:?}", ast_printer.print(Rc::clone(&ast)));
                println!("Evaluated: {:?}", interpreter.evaluate(ast));
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
