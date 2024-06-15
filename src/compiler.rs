use std::fs;

pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Expression {
    Vec<Instruction> i;
}

pub struct Statement {
    Vec<Expression> e;
}

pub struct Program {
    Vec<Statement> s;
}

pub fn compile(filename: String) -> std::io::Result<Program> {
    fs::open()
    // todo
    Ok(Program{})
}
