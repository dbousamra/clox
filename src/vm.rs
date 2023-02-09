use crate::chunk::{Chunk, OpCode};
use crate::compiler::Parser;
use crate::debug::{disassemble_instruction, print_value};
use crate::stack::Stack;
use crate::value::Value;
use std::fs;
use std::process::exit;

#[derive(PartialEq, Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Stack,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            stack: Stack::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut parser = Parser::new(source);

        if !parser.compile() {
            return InterpretResult::CompileError;
        }

        self.chunk = parser.chunk;
        self.ip = 0; // or self.chunk.code?

        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            // self.debug_trace_execution();
            let opcode = &self.chunk.code[self.ip];

            match opcode {
                OpCode::Return => {
                    print_value(&self.stack.pop());
                    println!();
                    return InterpretResult::Ok;
                }
                OpCode::Constant(index) => {
                    let constant = &self.chunk.constants.values[*index as usize];
                    self.stack.push(*constant);
                }
                OpCode::Negate => match self.stack.peek(0) {
                    Value::Number(val) => {
                        let neg_val = -val;
                        self.stack.pop();
                        self.stack.push(Value::Number(neg_val));
                    }
                },
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
            }
            self.ip += 1;
        }
    }

    pub fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(path).expect("Could not open file");
        let result = self.interpret(source.as_str());

        match result {
            InterpretResult::CompileError => exit(65),
            InterpretResult::RuntimeError => exit(70),
            InterpretResult::Ok => exit(0),
        }
    }

    pub fn debug_trace_execution(&self) {
        println!("          ");
        for slot in &self.stack.values {
            print!("[ ");
            print!("{:?}", slot);
            print!(" ]");
        }
        println!("");

        disassemble_instruction(&self.chunk, self.ip);
    }

    fn binary_op(&mut self, f: fn(f64, f64) -> f64) {
        let b = self.stack.pop();
        let a = self.stack.pop();

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                let result = f(a, b);
                self.stack.push(Value::Number(result));
            } // _ => return self.runtime_error("Operands must be numbers."),
        }
    }
}
