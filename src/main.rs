use std::process::Command;
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
                    let tex_path = path.with_extension("tex");
                    match fs::write(&tex_path, program.into_tex()) {
                        Ok(()) => {
                            match Command::new("pdflatex").args(&[
                                "-output-directory",
                                tex_path.parent().unwrap().to_str().unwrap(),
                                tex_path.to_str().unwrap()
                            ]).output() {
                                Ok(_)  => (),
                                Err(err) => {
                                    eprintln!("{err}");
                                    return;
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("{err}");
                            return;
                        }
                    }
                    interpret(&program);
                },
                Err(err) => eprintln!("{err}"),
            }
        },
        Err(err) => eprintln!("{err}"),
    }
}
