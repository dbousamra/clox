use std::io::{self, stdout, Write};

use crate::vm::VM;

pub fn repl(vm: &mut VM) {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        match stdin.read_line(&mut buffer) {
            Ok(0) | Err(_) => {
                println!();
                break;
            }
            Ok(_) => {
                vm.interpret(&buffer);
            }
        }
        buffer.clear()
    }
}
