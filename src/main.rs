use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::vm::VM;

mod chunk;
mod debug;
mod stack;
mod value;
mod vm;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    // add the constant value itself to the chunk’s constant pool
    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant(constant.try_into().unwrap()), 123);

    let constant = chunk.add_constant(Value::Number(3.4));
    chunk.write(OpCode::Constant(constant.try_into().unwrap()), 123);

    chunk.write(OpCode::Add, 123);

    let constant = chunk.add_constant(Value::Number(5.6));
    chunk.write(OpCode::Constant(constant.try_into().unwrap()), 123);

    chunk.write(OpCode::Divide, 123);
    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return, 123);

    // disassemble_chunk(&chunk, "test chunk");

    vm.interpret(chunk);
}
