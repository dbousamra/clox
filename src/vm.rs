use crate::chunk::{Chunk, OpCode};
use crate::debug::{disassemble_instruction, print_value};
use crate::stack::Stack;
use crate::value::Value;

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

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            self.debug_trace_execution();
            let opcode = &self.chunk.code[self.ip];

            match opcode {
                OpCode::Return => {
                    print_value(&self.stack.pop());
                    print!("\n");
                    return InterpretResult::Ok;
                }
                OpCode::Constant(index) => {
                    let constant = &self.chunk.constants.values[*index as usize];
                    print_value(constant);
                    self.stack.push(*constant);
                    print!("\n");
                }
                OpCode::Negate => {
                    match self.stack.peek(0) {
                        Value::Number(val) => {
                            let neg_val = -val;
                            self.stack.pop();
                            self.stack.push(Value::Number(neg_val));
                        } // _ => return self.runtime_error("Operand must be a number."),
                    }
                }
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
            }
            self.ip += 1;
        }
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
}
