use std::{fmt::{self, Display, Formatter}, str::FromStr};
use regex::Regex;

#[derive(Debug)]
pub enum Symbol {
    OpenBrace,
    CloseBrace,
    Grave,
    Squig,
    Excla,
    At,
    Pound,
    Dollar,
    Percent,
    Pow,
    Amp,
    Mul,
    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    LessThan,
    GreaterThan,
    Comma,
    Dot,
    Div,
    Question,
    Color,
    Semi,
    Apost,
    Sub,
    Underscore,
    Equals,
    Add,
}

impl FromStr for Symbol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\\{" => Ok(Symbol::OpenBrace),
            "\\}" => Ok(Symbol::CloseBrace),
            "`" => Ok(Symbol::Grave),
            "~" => Ok(Symbol::Squig),
            "!" => Ok(Symbol::Excla),
            "@" => Ok(Symbol::At),
            "#" => Ok(Symbol::Pound),
            "$" => Ok(Symbol::Dollar),
            "%" => Ok(Symbol::Percent),
            "^" => Ok(Symbol::Pow),
            "&" => Ok(Symbol::Amp),
            "*" => Ok(Symbol::Mul),
            "(" => Ok(Symbol::OpenParen),
            ")" => Ok(Symbol::CloseParen),
            "[" => Ok(Symbol::OpenBrack),
            "]" => Ok(Symbol::CloseBrack),
            "<" => Ok(Symbol::LessThan),
            ">" => Ok(Symbol::GreaterThan),
            "," => Ok(Symbol::Comma),
            "." => Ok(Symbol::Dot),
            "/" => Ok(Symbol::Div),
            "?" => Ok(Symbol::Question),
            ":" => Ok(Symbol::Color),
            ";" => Ok(Symbol::Semi),
            "'" => Ok(Symbol::Apost),
            "-" => Ok(Symbol::Sub),
            "_" => Ok(Symbol::Underscore),
            "=" => Ok(Symbol::Equals),
            "+" => Ok(Symbol::Add),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Macro {
    Times,
    Div,
    Frac,
}

impl FromStr for Macro {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\\times" => Ok(Macro::Times),
            "\\div"   => Ok(Macro::Div),
            "\\frac"  => Ok(Macro::Frac),
            _ => Err(format!("\"{s}\" is not a recognized macro.")),
        }
    }
}

#[derive(Debug)]
pub enum Scope {
    Pop,
    Push,
}

pub enum Instruction {
    Symbol(Symbol),
    Macro (Macro ),
    Var   (String),
    Num   (i32   ),
    Scope (Scope ),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Instruction::Symbol(x) => format!("Symbol({x:?})"),
            Instruction::Macro(x)  => format!( "Macro({x:?})"),
            Instruction::Var(x)    => format!(   "Var({x:?})"),
            Instruction::Num(x)    => format!(   "Num({x:?})"),
            Instruction::Scope(x)  => format!( "Scope({x:?})"),
        })
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_macro = Regex::new(r"\\[A-Za-z]+").unwrap();
        let re_op = Regex::new(r"\\\{|\\\}|[`~!@#$%\^&*()[\\]<>,./?:;'|\-_=+]").unwrap();
        let re_num = Regex::new(r"\d+\.?\d*|\.\d+").unwrap();
        let re_var = Regex::new(r"[A-Za-z]'*").unwrap();

        if re_macro.is_match(s) {
            match Macro::from_str(s){
                Ok(m) => Ok(Instruction::Macro(m)),
                Err(err) => Err(err),
            }
        } else if re_op.is_match(s) {
            Ok(Instruction::Symbol(Symbol::from_str(s).unwrap()))
        } else if re_num.is_match(s) {
            Ok(Instruction::Num(i32::from_str(s).unwrap()))
        } else if re_var.is_match(s) {
            Ok(Instruction::Var(s.into()))
        } else {
            Err("Unexpected Pattern".to_owned())
        }
    }
}

pub struct Expression {
    instructions: Vec<Instruction>,
}

impl Expression {
    fn new() -> Self {
        Self { instructions: Vec::new() }
    }
}

pub struct Statement {
    expressions: Vec<Expression>,
}

impl Statement {
    fn new() -> Self {
        Self { expressions: Vec::new() }
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    fn new() -> Self {
        Self { statements: Vec::new() }
    }

    fn push_statement(&mut self) {
        self.statements.push(Statement::new());
    }

    fn push_expression(&mut self) {
        let statement_index = self.statements.len() - 1;
        self.statements[statement_index].expressions.push(Expression::new());
    }

    fn push_instruction(&mut self, instruction: Instruction) {
        let statement_index = self.statements.len() - 1;
        let expression_index = self.statements[statement_index].expressions.len() - 1;
        self
            .statements[statement_index]
            .expressions[expression_index]
            .instructions.push(instruction);
    }

    fn clean(&mut self) {
        let statement_index = self.statements.len() - 1;
        let expression_index = self.statements[statement_index].expressions.len() - 1;
        if self.statements[statement_index].expressions[expression_index].instructions.len() == 0 {
            self.statements.pop();
        }
    }

    pub fn into_tex(&self) -> String {
        let mut result: String = "\\documentclass{article}\n\\usepackage{geometry}\n\\usepackage{graphicx}\n\\usepackage{amssymb}\n\\usepackage{amsmath}\n\\usepackage{amsthm}\n\\usepackage{empheq}\n\\usepackage{mdframed}\n\\usepackage{booktabs}\n\\usepackage{lipsum}\n\\usepackage{graphicx}\n\\usepackage{color}\n\\usepackage{psfrag}\n\\usepackage{bm}\n\\begin{document}\n\\begin{align*}\n".into();
        for statement in &self.statements {
            for expression in &statement.expressions {
                result.push_str("&");
                for instruction in &expression.instructions {
                    let instruction_string: String = match instruction {
                        Instruction::Symbol(x)
                            => match x {
                                Symbol::OpenBrace   => "\\{".into(),
                                Symbol::CloseBrace  => "\\}".into(),
                                Symbol::Grave       => "`".into(),
                                Symbol::Squig       => "~".into(),
                                Symbol::Excla       => "!".into(),
                                Symbol::At          => "@".into(),
                                Symbol::Pound       => "#".into(),
                                Symbol::Dollar      => "$".into(),
                                Symbol::Percent     => "%".into(),
                                Symbol::Pow         => "^".into(),
                                Symbol::Amp         => "&".into(),
                                Symbol::Mul         => "*".into(),
                                Symbol::OpenParen   => "(".into(),
                                Symbol::CloseParen  => ")".into(),
                                Symbol::OpenBrack   => "[".into(),
                                Symbol::CloseBrack  => "]".into(),
                                Symbol::LessThan    => "<".into(),
                                Symbol::GreaterThan => ">".into(),
                                Symbol::Comma       => ",".into(),
                                Symbol::Dot         => ".".into(),
                                Symbol::Div         => "/".into(),
                                Symbol::Question    => "?".into(),
                                Symbol::Color       => ":".into(),
                                Symbol::Semi        => ";".into(),
                                Symbol::Apost       => "'".into(),
                                Symbol::Sub         => "-".into(),
                                Symbol::Underscore  => "_".into(),
                                Symbol::Equals      => "=".into(),
                                Symbol::Add         => "+".into(),
                            },
                        Instruction::Macro (x)
                            => match x {
                                Macro::Times => "\\times".into(),
                                Macro::Div   => "\\div"  .into(),
                                Macro::Frac  => "\\frac" .into(),
                            },
                        Instruction::Var   (x)
                            => format!("{}", x),
                        Instruction::Num   (x)
                            => format!("{}", x),
                        Instruction::Scope (x)
                            => match x {
                                Scope::Pop  => "}".into(),
                                Scope::Push => "{".into(),
                            },
                    };
                    result.push_str(&instruction_string);
                }
            }
            result.push_str("\\\\\n");
        }
        result.push_str("\\end{align*}\n\\end{document}");
        result
    }
}

pub fn compile(code: String) -> Result<Program,String> {
    let re = Regex::new(r"(\d+\.?\d*|\.\d+|[A-Za-z]'*|\\\{|\\\}|\\\\|\\[A-Za-z]+|[`~!@#$%^&*()[\\]{}<>,./?:;'|\-_=+])").unwrap();

    let mut program = Program::new();
    program.statements.push(Statement::new());
    program.statements[0].expressions.push(Expression::new());
    for captures in re.captures_iter(&code) {
        let word = captures.get(0).unwrap().as_str();
        print!("{word}: ");
        match word {
            ";" => {
                println!("[END OF STATEMENT]");
                program.push_statement();
                program.push_expression();
            },
            "{" => {
                println!("[PUSH SCOPE]");
                program.push_instruction(Instruction::Scope(Scope::Push));
            },
            "}" => {
                println!("[POP SCOPE]");
                program.push_instruction(Instruction::Scope(Scope::Pop));
            },
            _ => {
                match Instruction::from_str(word) {
                    Ok(instruction) => {
                        println!("[{instruction}] {word}");
                        program.push_instruction(instruction);
                    }
                    Err(err) => return Err(err),
                }
            },
        }
    }
    program.clean();
    Ok(program)
}
