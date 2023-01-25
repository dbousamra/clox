use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }

    let instruction = &chunk.code[offset];

    match instruction {
        OpCode::Return => simple_instruction("OP_RETURN", offset),
        OpCode::Constant(index) => {
            constant_instruction("OP_CONSTANT", chunk, offset, (*index).into())
        }
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

fn print_value(value: &Value) {
    match value {
        Value::Number(n) => print!("number: {:?}", n),
    }
}
