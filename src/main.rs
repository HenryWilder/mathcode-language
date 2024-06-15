use std::{fs, env, path::Path};

pub mod compiler;
pub mod interpreter;

use crate::compiler::compile;
use crate::interpreter::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments. Need a file to compile.");
        return;
    }
    let filename = &args[1];
    println!("Compiling \"{filename}\"...");
    let path = Path::new(filename);
    match fs::read_to_string(path) {
        Ok(code) => {
            match compile(code) {
                Ok(program) => {
                    if let Err(err) = fs::write(path.with_extension("tex"), program.into_tex()) {
                        eprintln!("{err}");
                        return;
                    }
                    interpret(&program);
                },
                Err(err) => eprintln!("{err}"),
            }
        },
        Err(err) => eprintln!("{err}"),
    }
}
