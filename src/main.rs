use std::{env,path::Path};

pub mod compiler;
pub mod interpreter;

use crate::compiler::compile;
use crate::interpreter::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments. Need file to compile.");
        return;
    }
    let filename = &args[1];
    println!("Compiling \"{filename}\"...");
    let path = Path::new(filename);
    match compile(path) {
        Ok(program) => interpret(&program),
        Err(err) => eprintln!("{err}"),
    }
}
