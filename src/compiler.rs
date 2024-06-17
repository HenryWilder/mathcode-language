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
            r"\{" => Ok(Symbol::OpenBrace),
            r"\}" => Ok(Symbol::CloseBrace),
            "`"   => Ok(Symbol::Grave),
            "~"   => Ok(Symbol::Squig),
            "!"   => Ok(Symbol::Excla),
            "@"   => Ok(Symbol::At),
            "#"   => Ok(Symbol::Pound),
            "$"   => Ok(Symbol::Dollar),
            "%"   => Ok(Symbol::Percent),
            "^"   => Ok(Symbol::Pow),
            "&"   => Ok(Symbol::Amp),
            "*"   => Ok(Symbol::Mul),
            "("   => Ok(Symbol::OpenParen),
            ")"   => Ok(Symbol::CloseParen),
            "["   => Ok(Symbol::OpenBrack),
            "]"   => Ok(Symbol::CloseBrack),
            "<"   => Ok(Symbol::LessThan),
            ">"   => Ok(Symbol::GreaterThan),
            ","   => Ok(Symbol::Comma),
            "."   => Ok(Symbol::Dot),
            "/"   => Ok(Symbol::Div),
            "?"   => Ok(Symbol::Question),
            ":"   => Ok(Symbol::Color),
            ";"   => Ok(Symbol::Semi),
            "'"   => Ok(Symbol::Apost),
            "-"   => Ok(Symbol::Sub),
            "_"   => Ok(Symbol::Underscore),
            "="   => Ok(Symbol::Equals),
            "+"   => Ok(Symbol::Add),
            _ => Err(()),
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            &Symbol::OpenBrace   => r"\{".to_owned(),
            &Symbol::CloseBrace  => r"\}".to_owned(),
            &Symbol::Grave       => "`".to_owned(),
            &Symbol::Squig       => "~".to_owned(),
            &Symbol::Excla       => "!".to_owned(),
            &Symbol::At          => "@".to_owned(),
            &Symbol::Pound       => "#".to_owned(),
            &Symbol::Dollar      => "$".to_owned(),
            &Symbol::Percent     => "%".to_owned(),
            &Symbol::Pow         => "^".to_owned(),
            &Symbol::Amp         => "&".to_owned(),
            &Symbol::Mul         => "*".to_owned(),
            &Symbol::OpenParen   => "(".to_owned(),
            &Symbol::CloseParen  => ")".to_owned(),
            &Symbol::OpenBrack   => "[".to_owned(),
            &Symbol::CloseBrack  => "]".to_owned(),
            &Symbol::LessThan    => "<".to_owned(),
            &Symbol::GreaterThan => ">".to_owned(),
            &Symbol::Comma       => ",".to_owned(),
            &Symbol::Dot         => ".".to_owned(),
            &Symbol::Div         => "/".to_owned(),
            &Symbol::Question    => "?".to_owned(),
            &Symbol::Color       => ":".to_owned(),
            &Symbol::Semi        => ";".to_owned(),
            &Symbol::Apost       => "'".to_owned(),
            &Symbol::Sub         => "-".to_owned(),
            &Symbol::Underscore  => "_".to_owned(),
            &Symbol::Equals      => "=".to_owned(),
            &Symbol::Add         => "+".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Macro {
    Comment,
    Times,
    Div,
    Frac,
    Print,
    Input,
    Limit,
}

impl FromStr for Macro {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"\comment" => Ok(Macro::Comment),
            r"\times"   => Ok(Macro::Times),
            r"\div"     => Ok(Macro::Div),
            r"\frac"    => Ok(Macro::Frac),
            r"\print"   => Ok(Macro::Print),
            r"\input"   => Ok(Macro::Input),
            r"\lim"     => Ok(Macro::Limit),
            _ => Err(format!("\"{s}\" is not a recognized macro.")),
        }
    }
}

impl ToString for Macro {
    fn to_string(&self) -> String {
        match self {
            &Macro::Comment => r"\comment".to_owned(),
            &Macro::Times   => r"\times"  .to_owned(),
            &Macro::Div     => r"\div"    .to_owned(),
            &Macro::Frac    => r"\frac"   .to_owned(),
            &Macro::Print   => r"\print"  .to_owned(),
            &Macro::Input   => r"\input"  .to_owned(),
            &Macro::Limit   => r"\lim"    .to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Scope {
    Pop,
    Push,
}

#[derive(Debug)]
pub enum Comment {
    Block(String),
    Annotation(String),
    Inline(String),
}

pub enum Instruction {
    Symbol (Symbol ),
    Macro  (Macro  ),
    Var    (String ),
    Num    (i32    ),
    Scope  (Scope  ),
    Newline,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Instruction::Symbol(x)  => format!("Symbol({x:?})"),
            Instruction::Macro(x)   => format!( "Macro({x:?})"),
            Instruction::Var(x)     => format!(   "Var({x:?})"),
            Instruction::Num(x)     => format!(   "Num({x:?})"),
            Instruction::Scope(x)   => format!( "Scope({x:?})"),
            Instruction::Newline    => "Newline".to_owned(),
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
        } else if re_var.is_match(s) {
            Ok(Instruction::Var(s.into()))
        } else if re_op.is_match(s) {
            match Symbol::from_str(s) {
                Ok(sym) => Ok(Instruction::Symbol(sym)),
                Err(()) => Err(format!("\"{s}\" is not a recognized symbol")),
            }
        } else if re_num.is_match(s) {
            Ok(Instruction::Num(i32::from_str(s).unwrap()))
        } else {
            Err(format!("Unsure what \"{s}\" means"))
        }
    }
}

pub struct Statement {
    instructions: Vec<Instruction>,
}

impl Statement {
    fn new() -> Self {
        Self { instructions: Vec::new() }
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    fn new() -> Self {
        Self { statements: Vec::new() }
    }

    fn is_current_statement_empty(&self) -> Option<bool> {
        self.statements.last().map(|last| last.instructions.is_empty())
    }

    fn push_statement(&mut self) {
        self.statements.push(Statement::new());
    }

    fn push_instruction(&mut self, instruction: Instruction) {
        if self.statements.len() < 1 {
            self.push_statement();
        }
        self.statements.last_mut().unwrap().instructions.push(instruction);
    }

    pub fn into_tex(&self) -> String {
        let mut result: String =
r"\documentclass{article}
\usepackage[dvipsnames]{xcolor}
\usepackage{graphicx,amssymb,amsmath,amsthm,empheq,mdframed,color,bm}
\newcommand\print{\text{print}}
\newcommand\comment[1]{~{\color{ForestGreen}\text{#1}}~}
\begin{document}
\begin{align*}
".into();
        for statement in &self.statements {
            // result.push_str(r"&");
            for instruction in &statement.instructions {
                let instruction_string: String = match instruction {
                    Instruction::Symbol(x) => x.to_string(),
                    Instruction::Macro (x) => x.to_string(),
                    Instruction::Var   (x) => format!("{x}"),
                    Instruction::Num   (x) => format!("{x}"),
                    Instruction::Scope (x) => match x {
                        &Scope::Pop  => "}".into(),
                        &Scope::Push => "{".into(),
                    },
                    Instruction::Newline => r"\\".into(),
                };
                result.push_str(&instruction_string);
            }
            result.push_str(concat!(r"\\", "\n"));
        }
        result.push_str(
r"\end{align*}
\end{document}");
        result
    }
}

pub fn compile(code: String) -> Result<Program,String> {
    let re = Regex::new(r"(%.*?\n|\d+\.?\d*|\.\d+|[A-Za-z]'*|\\\{|\\\}|\\\\|\\[A-Za-z]+|[`~!@#$%^&*()[\\]{}<>,./?:;'|\-_=+])").unwrap();

    let mut program = Program::new();
    program.push_statement();
    for captures in re.captures_iter(&code) {
        let word = captures.get(0).unwrap().as_str().trim();
        print!("\"{word}\": ");
        if word.starts_with('%') {
            println!("[COMMENT]");
            continue;
        }
        match word {
            ";" => {
                if program.is_current_statement_empty().unwrap_or(true) {
                    println!("[END OF (empty) STATEMENT (ignored)]");
                } else {
                    println!("[END OF STATEMENT]");
                    program.push_statement();
                }
            },
            "{" => {
                println!("[PUSH SCOPE]");
                program.push_instruction(Instruction::Scope(Scope::Push));
            },
            "}" => {
                println!("[POP SCOPE]");
                program.push_instruction(Instruction::Scope(Scope::Pop));
            },
            r"\\" => {
                println!("[NEWLINE]");
                program.push_instruction(Instruction::Newline);
            }
            _ => {
                match Instruction::from_str(word) {
                    Ok(instruction) => {
                        println!("[{instruction}]");
                        program.push_instruction(instruction);
                    }
                    Err(err) => return Err(err),
                }
            },
        }
    }
    Ok(program)
}
