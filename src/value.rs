#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f64),
}

pub struct ValuePool {
    pub values: Vec<Value>,
}

impl ValuePool {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, value: Value) {
        self.values.push(value);
    }
}
