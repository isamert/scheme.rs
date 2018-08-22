use std::io;
use std::io::prelude::*;

use lexer;
use parser;
use env::EnvRef;
use env::EnvRefT;

pub fn run(env: &EnvRef) {
    let mut i = 0;

    loop {
        let mut line = String::new();
        io::stdout().write(b"scheme.rs> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        let tokens = lexer::tokenize(&mut line.chars().peekable());
        let sexprs = parser::parse(tokens);

        match sexprs {
            Ok(sexprs) => {
                for sexpr in sexprs {
                    let evaluated = sexpr.eval(env);

                    match evaluated {
                        Ok(evaluated) => {
                            if !evaluated.is_unspecified() {
                                println!("${} = {}", i, evaluated);
                                env.define(format!("${}", i), evaluated);
                                i += 1;
                            }
                        },
                        Err(e) => println!("{}", e)
                    }
                }
            },
            Err(e) => println!("{}", e)
        }
    }
}
