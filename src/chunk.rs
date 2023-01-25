use crate::value::{Value, ValuePool};

pub enum OpCode {
    Return,
    Constant(u8), // index into the constant pool
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValuePool,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValuePool::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.add(value);
        self.constants.values.len() - 1
    }
}
