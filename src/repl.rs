use std::io;

use lexer;
use parser;
use env::EnvRef;
use env::EnvRefT;

pub fn run(env: EnvRef) {
    let stdin = io::stdin();

    let mut i = 0;
    let mut line = String::new();
    loop {
        let _size = stdin.read_line(&mut line);
        let tokens = lexer::tokenize(&line);
        println!("TOKENS: {:?}", tokens);
        let sexprs = parser::parse(tokens);
        for sexpr in sexprs.iter() {
            println!("NONEVALED: {:?}", sexpr);
            let evaluated = sexpr.eval(&env);
            println!("${} = {}", i, evaluated);
            // Add $i to environment so user can use the currently evaluated value
            env.define(format!("${}", i), evaluated);
            i += 1;
        }
    }
}
