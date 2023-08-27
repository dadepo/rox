use crate::chunk::OpCode::OpReturn;
use anyhow::anyhow;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl TryFrom<&u8> for OpCode {
    type Error = anyhow::Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpConstant),
            1 => Ok(OpReturn),
            _ => Err(anyhow!("No enum variant for {value}")),
        }
    }
}

/// Chunk has a constant field which when a constant
/// is added via add_const it returns the index of the constant
#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constant: Vec<f64>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    /// Adds the opcode to the code byte array, and also the corresponding
    /// line the code exist in the source
    pub fn write(&mut self, code: u8, line: u32) -> () {
        self.code.push(code);
        self.lines.push(line);
    }

    /// Adds a constant to the constant pool and return the index
    // TDOO optimise for same value using same index
    //
    pub fn add_const(&mut self, constant: f64) -> u8 {
        if self.constant.len() + 1 > 256 {
            panic!("Constant pool currently can support only 255 constants")
        }
        self.constant.push(constant);
        (self.constant.len() - 1) as u8
    }
}
