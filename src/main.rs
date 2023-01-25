use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::value::Value;
use std::convert::TryFrom;

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new();
    let index = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant(u8::try_from(index).unwrap()), 123);
    chunk.write(OpCode::Return, 123);

    disassemble_chunk(&chunk, "test chunk");
}
