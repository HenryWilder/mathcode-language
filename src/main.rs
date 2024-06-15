use std::env;

pub mod compiler;
pub mod interpreter;

use crate::compiler::{Program,compile};
use crate::interpreter::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    if args.len() != 2 {
        println!("Not enough arguments. Expected 1, got {}", args.len() - 1);
        return;
    }
    let filename = args[1];
    let compiled: Program = compile(filename);
    interpret(&compiled);
}
