use std::{env, fs, process};
use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::vm::VM;
use rustyline::DefaultEditor;
use std::rc::Rc;
use rustyline::error::ReadlineError;

mod chunk;
mod debug;
mod vm;
mod compiler;
mod scanner;

fn main() -> anyhow::Result<()> {

    let mut args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 1 {
        println!("Usage: rox [script]");
        process::exit(1);
    }

    let mut vm = VM::new();

    if args.len() == 1 {
        let file_content = fs::read_to_string(args.remove(0))?;
        vm.interpret(&file_content);

    } else {
        let mut rl = DefaultEditor::new()?;
        rl.load_history("history_rox.txt").ok();

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    vm.interpret(&line);
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
        rl.save_history("history_rox.txt").ok();
    }

    Ok(())
}
