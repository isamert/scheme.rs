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
        println!("TOKENS: {:?}", tokens);

        let sexprs = parser::parse(tokens);
        for sexpr in sexprs {
            println!("NONEVALED: {:?}", sexpr);
            let evaluated = sexpr.eval(&env);
            println!("${} = {}", i, evaluated);

            // Add $i to environment so user can use the currently evaluated value
            env.define(format!("${}", i), evaluated);
            i += 1;
        }
    }
}
