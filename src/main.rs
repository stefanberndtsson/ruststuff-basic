#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod parser;

fn main() {
//    parser::run("10 LET A$ = \"MY TEXT\"\n20 PRINT \"HELLO\"\n30 PRINT A$\n40 GOTO 40\n");
    parser::run("10 LET A = 129\n20 PRINT \"HELLO\"\n30 PRINT A\n40 GOTO 40\n");
//    parser::run("10 LET A$=\"MY TEXT\"\n");
}
