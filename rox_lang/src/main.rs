use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::vm::VM;
use std::rc::Rc;

mod chunk;
mod debug;
mod vm;

fn main() -> anyhow::Result<()> {
    let mut chunk: Chunk = Chunk::new();

    let constant_index = chunk.add_const(1.2);

    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant_index, 123);
    chunk.write(OpCode::OpReturn as u8, 123);

    disassemble_chunk(&chunk, "test chunk")?;
    println!("== End manual disassemble ==");

    let mut vm = VM::new();
    vm.interpret(Rc::new(chunk));

    Ok(())
}
