use crate::compiler::Program;

pub fn interpret(program: &Program) {
    for statement in &program.statements {
        for instruction in &statement.instructions {
            println!("{}", instruction.to_string());
        }
    }
}
