use std::io;
use std::io::prelude::*;

use lexer;
use parser;
use env::EnvRef;
use env::EnvRefT;

pub fn run(env: EnvRef) {
    let stdin = io::stdin();

    let mut i = 0;
    for line in stdin.lock().lines() {
        let tokens = lexer::tokenize(&line.unwrap());
        let sexprs = parser::parse(tokens);

        for sexpr in sexprs {
            let evaluated = sexpr.eval(&env);

            if !evaluated.is_unspecified() {
                println!("${} = {}", i, evaluated);
                env.define(format!("${}", i), evaluated);
            }

            i += 1;
        }
    }
}
