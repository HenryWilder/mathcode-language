use crate::compiler::{Program, Statement};
use std::collections::HashMap;

pub fn interpret(program: &Program) {
    let mut vars: HashMap<String, Statement> = HashMap::new();

    for statement in &program.statements {
        for instruction in &statement.instructions {
            println!("{}", instruction.to_string());
        }
    }
}
