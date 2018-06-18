#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod parser;

fn main() {
    let parsed = parser::parse("10 PRINT \"HELLO\"\n20 GOTO 10\n");
    println!("{:#?}", parsed);
    match parsed {
        Ok(p) => parser::execute(&p),
        _ => ()
    }
}
