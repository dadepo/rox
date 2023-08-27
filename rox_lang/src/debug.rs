use crate::chunk::{Chunk, OpCode};
use anyhow::anyhow;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) -> anyhow::Result<()> {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset)?;
    }
    Ok(())
}

/// returns the offset of the next opcode
/// Prints ByteOffset SourceOffset Opcode Operand
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> anyhow::Result<usize> {
    // The offset in the byte code
    print!("{offset:04}");
    // The corresponding line of the byte code in source code
    if offset > 0 && chunk.lines.get(offset) == chunk.lines.get(offset - 1) {
        print!(" | ");
    } else {
        print!(
            "{:>4} ",
            chunk
                .lines
                .get(offset)
                .ok_or(anyhow!("Line value not found"))?
        )
    }

    match chunk.code.get(offset) {
        None => Err(anyhow!("No op code at given offset {offset}")),
        Some(code) => {
            match code {
                _ if *code == OpCode::OpReturn as u8 => {
                    println!("{:?}", OpCode::OpReturn);
                    Ok(offset + 1_usize)
                }
                _ if *code == OpCode::OpConstant as u8 => {
                    // Get the index of the operand in the adjacent index
                    let constant_index = chunk
                        .code
                        .get(offset + 1)
                        .ok_or(anyhow!("Constant index not found"))?;

                    print!("{:<16?} {:>4} ", OpCode::OpConstant, constant_index);
                    println!(
                        "'{}'",
                        chunk
                            .constant
                            .get(*constant_index as usize)
                            .ok_or(anyhow!("Constant value not found"))?
                    );
                    Ok(offset + 2_usize)
                }
                _ => Err(anyhow!(
                    "Unrecognized op code {code} at given offset {offset}"
                )),
            }
        }
    }
}
