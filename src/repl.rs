use std::io;
use std::io::prelude::*;

use lexer;
use lexer::Token;
use parser;
use parser::SExpr;
use evaluator;
use evaluator::Args;
use env::EnvRef;
use env::EnvRefT;
use primitives::lang;

pub fn run(env: EnvRef) {
    let stdin = io::stdin();

    let mut i = 0;
    for line in stdin.lock().lines() {
        let sexprs = parser::parse(lexer::tokenize(&line.unwrap()));
        for sexpr in sexprs.iter() {
            let evaluated = evaluator::eval(sexpr, env.clone_ref());
            println!("${} = {}", i, evaluated);

            // TODO: create an `args!` macro

            // Add $i to environment so user can use the currently evaluated value
            lang::define(Args::new(
                vec![SExpr::Atom(Token::Symbol(format!("${}", i))), evaluated],
                env.clone_ref()
            ));

            i += 1;
        }
    }
}
