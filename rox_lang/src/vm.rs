use crate::chunk::{Chunk, OpCode};
use crate::vm::InterpretResult::InterpretOk;
use anyhow::anyhow;
use std::rc::Rc;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    pub chunk: Rc<Chunk>,
    pub ip: u8,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Rc::new(Chunk::default()),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Rc<Chunk>) -> InterpretResult {
        self.chunk = chunk;
        match self.run() {
            Ok(result) => result,
            Err(_) => InterpretResult::InterpretRuntimeError,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<InterpretResult> {
        loop {
            let ip = self.get_next_ip();
            let instruction: OpCode = self
                .chunk
                .code
                .get(ip)
                .ok_or(anyhow!("No instruction found at index"))?
                .try_into()?;
            match instruction {
                OpCode::OpConstant => {
                    let ip = self.get_next_ip();
                    let constant_index = self
                        .chunk
                        .code
                        .get(ip)
                        .ok_or(anyhow!("No instruction found at index"))?;
                    let constant_value = self
                        .chunk
                        .constant
                        .get(*constant_index as usize)
                        .ok_or(anyhow!("No constnt value found at index"))?;
                    println!("'{}'", constant_value);
                }
                OpCode::OpReturn => return Ok(InterpretOk),
            }
        }
    }

    fn get_next_ip(&mut self) -> usize {
        // get's the current value of self.ip, which is index to operate on next
        // then increments that value
        self.ip += 1;
        (self.ip - 1) as usize
    }
}