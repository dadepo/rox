mod scanner;
mod token;
mod predicate;
mod expr;
mod parser;

use crate::scanner::run;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::{env, process};
use crate::parser::Parser;

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
                println!("Line: {}", line);
                let tokens = run(line).unwrap();
                let mut parser = Parser::new(tokens);
                let ast = parser.parse().unwrap();
                println!("Tokens: {:?}", ast);
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
