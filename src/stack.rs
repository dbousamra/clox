use crate::value::Value;

const STACK_SIZE: usize = 256;

pub struct Stack {
    pub values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("Empty stack")
    }

    pub fn peek(&self, distance: usize) -> &Value {
        self.values
            .get(self.values.len() - 1 - distance)
            .expect("Failed to peek")
    }
}
