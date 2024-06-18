use std::fmt::{self, Display, Formatter};
use regex::Regex;

pub trait IntoTex {
    fn into_tex(&self) -> String;
}

#[derive(Debug)]
pub enum ReqRel {
    In,
    NotIn,
}

impl Display for ReqRel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ReqRel::*;
        write!(f, "{}", match self {
            In    => r"\in",
            NotIn => r"\notin",
        })
    }
}

#[derive(Debug)]
pub struct Req {
    rel: ReqRel,
    val: Expr,
}

#[derive(Debug)]
pub enum ValRel {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

impl Display for ValRel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ValRel::*;
        write!(f, "{}", match self {
            EQ => "=",
            NE => r"\ne",
            LT => "<",
            LE => r"\le",
            GT => ">",
            GE => r"\ge",
        })
    }
}

#[derive(Debug)]
pub struct Def {
    rel: ValRel,
    val: Expr,
}

#[derive(Debug)]
pub enum Macro {
    Prompt{
        query: Option<Expr>,
    },
    Display{
        content: Expr,
    },
    LetVar{
        var: Expr,
        req: Option<Req>,
        def: Option<Def>,
    },
}

impl IntoTex for Macro {
    fn into_tex(&self) -> String {
        use Macro::*;
        match self {
            Prompt{
                query,
            } => format!(r"\prompt {}",
                query.map_or(String::new(), |v| format!("[{}]", v.into_tex())),
            ),

            Display{
                content,
            } => format!(r"\display {{{}}}",
                content.into_tex(),
            ),

            LetVar{
                var,
                req,
                def,
            } => format!(r"\letvar {} {} {}",
                var.into_tex(),
                req.map_or(String::new(), |v| format!("{} {}", v.rel, v.val.into_tex())),
                def.map_or(String::new(), |v| format!("{} {}", v.rel, v.val.into_tex())),
            ),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Cmd(Macro),
    Var(String),
}

impl IntoTex for Instruction {
    fn into_tex(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
/// Can be contained within commands
pub struct Expr {
    pub instructions: Vec<Instruction>
}

impl Expr {
    fn new() -> Expr {
        Expr{ instructions: Vec::new() }
    }
}

impl IntoTex for Expr {
    fn into_tex(&self) -> String {
        let mut result = String::new();
        for instruction in &self.instructions {
            result.push_str(instruction.into_tex().as_str());
        }
        result
    }
}

#[derive(Debug)]
pub struct Statement {
    pub expressions: Vec<Expr>,
}

impl Statement {
    fn new() -> Self {
        Self { expressions: Vec::new() }
    }
}

impl IntoTex for Statement {
    fn into_tex(&self) -> String {
        self.expressions.iter()
            .map(|expr| expr.into_tex())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn new() -> Self {
        Self { statements: Vec::new() }
    }

    fn end_statement(&mut self) {
        if self.statements.last().is_some_and(|s| s.expressions.last().is_some_and(|e| !e.instructions.is_empty())) {
            self.statements.push(Statement::new());
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        if self.statements.is_empty() {
            self.statements.push(Statement::new());
        }
        let current_statement = self.statements.last_mut().unwrap();

        if current_statement.expressions.is_empty() {
            current_statement.expressions.push(Expr::new());
        }
        let current_expression = current_statement.expressions.last_mut().unwrap();

        current_expression.instructions.push(instruction);
    }
}

impl IntoTex for Program {
    fn into_tex(&self) -> String {
        [
r"\documentclass{article}
\usepackage[dvipsnames]{xcolor}
\usepackage{graphicx,amssymb,amsmath,amsthm,empheq,mdframed,color,bm}
\newcommand\display[1]{\text{display}(#1)}
\newcommand\prompt[1]{\text{prompt}(#1)}
\newcommand\comment[1]{~{\color{ForestGreen}\text{#1}}~}
\newcommand\letvar{\text{Let }}
\newcommand\where{\text{if }}
\newcommand\owise{\text{otherwise}}
\begin{document}
\begin{align*}",
            self.statements.iter()
                .map(|stmnt| stmnt.into_tex())
                .collect::<Vec<String>>()
                .join(concat!(r"\\", "\n"))
                .as_str(),
r"
\end{align*}
\end{document}"
        ]
            .join("")
    }
}

pub fn compile_expr(code: String) -> Result<Expr,String> {

}

pub fn compile(code: String) -> Result<Program,String> {
    let re_token = Regex::new([
        r"%(?<comment>.*?)\n",
        r"\\letvar\b\s*(?<letvar>\S.*?)\s*(?:(?<letreq>\\in|\\notin)\s*(?<letreq_set>\S.*?))?\s*(?:(?<letdef_rel>\\ge|\\le|\\ne|=)\s*(?<letdef>\S.*?))?\s*\\\\",
    ].join("|").as_str()).unwrap();

    let mut program = Program::new();

    for segment in re_token.captures_iter(&code) {
        println!("segment: \"{}\"", segment.get(0).unwrap().as_str());

        if let Some(_) = segment.name("comment") {
            println!("[comment]");
        } else if let Some(var) = segment.name("letvar") {
            println!("[let {}]", var.as_str());
            program.add_instruction(Instruction::Cmd(Macro::LetVar{
                var: var.as_str(),
                req: ,
                def: ,
            }))
        }
    }

    Ok(program)
}
