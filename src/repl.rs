use std::io;
use std::io::prelude::*;

use lexer;
use parser;
use parser::SExpr;
use evaluator::ToArgs;
use env::EnvRef;
use env::EnvRefT;
use primitives::lang;

pub fn run(env: EnvRef) {
    let stdin = io::stdin();

    let mut i = 0;
    for line in stdin.lock().lines() {
        let tokens = lexer::tokenize(&line.unwrap());
        println!("TOKENS: {:?}", tokens);
        let sexprs = parser::parse(tokens);
        for sexpr in sexprs.iter() {
            println!("NONEVALED: {:?}", sexpr);
            let evaluated = sexpr.eval(&env);
            println!("${} = {}", i, evaluated);

            // TODO: create an `args!` macro

            // Add $i to environment so user can use the currently evaluated value
            env.define(format!("${}", i), evaluated);
            i += 1;
        }
    }
}
