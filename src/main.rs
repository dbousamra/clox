use crate::repl::repl;
use crate::vm::VM;
use std::env;
use std::process::exit;

mod chunk;
mod compiler;
mod debug;
mod repl;
mod scanner;
mod stack;
mod value;
mod vm;

fn main() {
    let mut argv = env::args();
    let mut vm = VM::new();

    match argv.len() {
        1 => repl(&mut vm),
        2 => vm.run_file(&argv.nth(1).expect("Could not parse argv")),
        _ => {
            eprintln!("Usage: clox [path]");
            exit(64);
        }
    }
}
