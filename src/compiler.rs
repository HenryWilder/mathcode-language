use std::{fs,path::Path};
use regex::Regex;

pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Var(String),
    Val(i64),
}

pub struct Expression {
    instructions: Vec<Instruction>,
}

impl Expression {
    pub fn new() -> Self {
        Self { instructions: Vec::new() }
    }
}

pub struct Statement {
    expressions: Vec<Expression>,
}

impl Statement {
    pub fn new() -> Self {
        Self { expressions: Vec::new() }
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
}

pub fn compile(path: &Path) -> std::io::Result<Program> {
    let re = Regex::new(r"((?:\d+\.?\d*|\.\d+)|[A-Za-z]|\\\{|\\\}|\\\\|\\[A-Za-z]+|[`~!@#$%^&*()[\\]{}<>,./?:;'|\-_=+])").unwrap();
    match fs::read_to_string(path) {
        Ok(code) => {
            let mut result = Program::new();
            result.statements.push(Statement::new());
            for captures in re.captures_iter(&code) {
                let word = captures.get(0).unwrap().as_str();
                match word {
                    ";" => {
                        println!("[END OF STATEMENT]");
                        result.statements.push(Statement::new());
                    },
                    _ => {
                        println!("{word}")
                    },
                }
            }
            Ok(result)
        },
        Err(err) => Err(err),
    }
}
