use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub fn print_value(value: &Value) {
    match value {
        Value::Number(n) => print!("{}", n),
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }

    let instruction = &chunk.code[offset];

    match instruction {
        OpCode::Return => simple_instruction("RETURN", offset),
        OpCode::Constant(index) => constant_instruction("CONSTANT", chunk, offset, (*index).into()),
        OpCode::Negate => simple_instruction("NEGATE", offset),
        OpCode::Add => simple_instruction("ADD", offset),
        OpCode::Subtract => simple_instruction("SUBTRACT", offset),
        OpCode::Multiply => simple_instruction("MULTIPLY", offset),
        OpCode::Divide => simple_instruction("DIVIDE", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize, index: usize) -> usize {
    print!("{} {:?} '", name, index);
    print_value(&chunk.constants.values[index]);
    println!("'");
    offset + 1
}
