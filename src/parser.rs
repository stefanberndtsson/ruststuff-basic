use self::arithmetic::code;
use std::collections::HashMap;

#[derive(Debug)]
enum Param {
    String(String)
}

#[derive(Debug)]
enum Stmt {
    Print(Param),
    Goto(u32),
    GotoIdx(usize)
}

#[derive(Debug)]
pub struct Line {
    linenum: u32,
    stmt: Stmt
}

peg_file! arithmetic("basic.peg");

pub fn parse(str: &str) -> Result<Vec<Line>,arithmetic::ParseError> {
    let lines = code(str);
    match lines {
        Ok(mut l) => { index_lines(&mut l); Ok(l) },
        Err(e) => Err(e)
    }
}

pub fn index_lines(lines: &mut Vec<Line>) {
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

pub fn execute(lines: &Vec<Line>) {
    let mut idx = 0;

    while idx <= lines.len() {
        idx = match lines[idx].stmt {
            Stmt::Print(ref param) => {
                match param {
                    Param::String(s) => println!("{}", s)
                };
                idx+1
            },
            Stmt::GotoIdx(newidx) => { newidx },
            _ => { idx+1 }
        }
    }
}
