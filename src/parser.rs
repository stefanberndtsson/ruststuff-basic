use self::arithmetic::code;
use std::collections::HashMap;

#[derive(Debug)]
enum Param {
    String(String),
    Num(i32),
    StrVar(String),
    NumVar(String)
}

#[derive(Debug)]
enum Stmt {
    Print(Param),
    Goto(u32),
    GotoIdx(usize),
    Let(Param,Param),
}

#[derive(Debug)]
struct Line {
    linenum: u32,
    stmt: Stmt
}

#[derive(Debug)]
pub struct Code {
    lines: Vec<Line>,
    variables: Variables
}

#[derive(Debug)]
struct Variables {
    strvars: HashMap<String,String>,
    numvars: HashMap<String,i32>
}

impl Code {
    fn new() -> Self {
        let vars = Variables {
            strvars: HashMap::new(),
            numvars: HashMap::new()
        };
        Code {
            lines: Vec::new(),
            variables: vars
        }
    }
}

peg_file! arithmetic("basic.peg");

fn parse(mut codestate: &mut Code, str: &str) -> Result<(),arithmetic::ParseError> {
    let lines = code(str, &mut codestate);
    match lines {
        Ok(()) => { index_lines(&mut codestate.lines); Ok(()) },
        Err(e) => Err(e)
    }
}

fn index_lines(lines: &mut Vec<Line>) {
    let mut lineindexes: HashMap<u32,usize> = HashMap::new();

    for (idx,line) in lines.iter().enumerate() {
        lineindexes.insert(line.linenum,idx);
    }

    for line in lines {
        match line.stmt {
            Stmt::Goto(linenum) => { line.stmt = Stmt::GotoIdx(*(lineindexes.get(&linenum).unwrap())) },
            _ => ()
        }
    }
}

fn execute(codestate: &mut Code) {
    let mut idx = 0;
    let mut idx_force: Option<usize> = None;

    while idx <= codestate.lines.len() {
        idx_force = None;
        match codestate.lines[idx].stmt {
            Stmt::Print(ref param) => { stmt_print(&codestate.variables, param) },
            Stmt::Let(ref var,ref data) => { stmt_let(&mut codestate.variables, &var, &data) },
            Stmt::GotoIdx(newidx) => { idx_force = Some(newidx) },
            ref err @ _ => { eprintln!("Unhandled Stmt: {:?}", err); }
        }
        idx = match idx_force {
            Some(newidx) => newidx,
            None => idx+1
        }
    }
}

fn stmt_let(variables: &mut Variables, var: &Param, data: &Param) {
    match var {
        Param::StrVar(v) => {
            let data = param_string(variables, data);
            variables.strvars.insert(v.to_string(), data);
        },
        Param::NumVar(v) => {
            let data = param_num(variables, data);
            variables.numvars.insert(v.to_string(), data);
        }
        err @ _ => { eprintln!("Unhandled Let: {:?} {:?}", err, data); }
    }
}

fn stmt_print(variables: &Variables, param: &Param) {
    println!("{}", param_string(variables, param));
}

fn param_num(variables: &Variables, param: &Param) -> i32 {
    match param {
        Param::Num(n) => *n,
        Param::NumVar(v) => {
            let varval = variables.numvars.get(v);
            match varval {
                Some(n) => *n,
                None => { panic!("Undefined variable: {}$", v);}
            }
        },
        err @ _ => { eprintln!("Unhandled param_num: {:?}", err); 0 }
    }
}

fn param_string(variables: &Variables, param: &Param) -> String {
    match param {
        Param::String(s) => s.to_string(),
        Param::StrVar(v) => {
            let varval = variables.strvars.get(v);
            match varval {
                Some(s) => s.to_string(),
                None => { panic!("Undefined variable: {}$", v);}
            }
        },
        Param::Num(n) => n.to_string(),
        Param::NumVar(v) => {
            let varval = variables.numvars.get(v);
            match varval {
                Some(n) => n.to_string(),
                None => { panic!("Undefined variable: {}", v);}
            }
        }
    }
}

pub fn run(code: &str) {
    let mut codestate = Code::new();
    
    let parsed = parse(&mut codestate, code);
    println!("{:#?}", codestate);
    match parsed {
        Ok(_) => execute(&mut codestate),
        Err(e) => println!("{:?}", e)
    }

}
