use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_instruction;
use crate::vm::InterpretResult::InterpretOk;
use anyhow::anyhow;
use std::ops::Deref;
use std::rc::Rc;
use crate::compiler::compile;

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    pub chunk: Rc<Chunk>,
    pub ip: u8,
    pub debug_trace_execution: bool,
    pub stack: Vec<f64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Rc::new(Chunk::default()),
            ip: 0,
            debug_trace_execution: true,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        compile(source);
        InterpretOk
    }

    pub fn run(&mut self) -> anyhow::Result<InterpretResult> {
        loop {
            if self.debug_trace_execution {
                print!("          ");
                for value in &self.stack {
                    print!("[");
                    print!("{}", value);
                    print!("]");
                }
                println!();

                disassemble_instruction(self.chunk.deref(), self.ip as usize)?;
            }

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
                        .ok_or(anyhow!("No constant value found at index"))?;
                    self.push(*constant_value);
                }
                OpCode::OpAdd => {
                    self.binary_op(BinaryOp::Add)?;
                }
                OpCode::OpSubtract => {
                    self.binary_op(BinaryOp::Subtract)?;
                }
                OpCode::OpMultiply => {
                    self.binary_op(BinaryOp::Multiply)?;
                }
                OpCode::OpDivide => {
                    self.binary_op(BinaryOp::Divide)?;
                }
                OpCode::OpNegate => {
                    let value = self.pop()?;
                    self.push(-value)
                }
                OpCode::OpReturn => {
                    println!("{}", self.pop()?);
                    return Ok(InterpretOk);
                }
            }
        }
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> anyhow::Result<f64> {
        self.stack.pop().ok_or(anyhow!("Cannot pop empty stack"))
    }

    fn binary_op(&mut self, op: BinaryOp) -> anyhow::Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        match op {
            BinaryOp::Add => {
                self.push(a + b);
            }
            BinaryOp::Subtract => {
                self.push(a - b);
            }
            BinaryOp::Multiply => {
                self.push(a * b);
            }
            BinaryOp::Divide => {
                self.push(a / b);
            }
        }
        Ok(())
    }

    fn get_next_ip(&mut self) -> usize {
        // get's the current value of self.ip, which is index to operate on next
        // then increments that value
        self.ip += 1;
        (self.ip - 1) as usize
    }
}
