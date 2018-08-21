#[macro_use]
mod serr;
#[macro_use]
mod utils;

mod env;
mod lexer;
mod parser;
mod expander;
mod port;
mod procedure;
mod evaluator;
mod primitives;
mod pretty_print;
mod repl;

use std::env::args;
use std::fs::read_to_string;

use env::Env;
use lexer::tokenize;
use parser::parse;

fn main() {
    let args = args().collect::<Vec<_>>();
    let env = Env::with_values(Env::null(), primitives::env()).into_ref();

    if args.len() == 1 {
        repl::run(&env);
    } else if args.len() == 2 {
        let path = &args[1];
        let scm = read_to_string(path).expect("Can't read file.");

        // TODO: run main function? (define (main args) ...)
        match parse(tokenize(&scm)) {
            Ok(sexprs) => {
                for sexpr in sexprs {
                    match sexpr.eval(&env) {
                        Ok(_) => (),
                        Err(e) => eprintln!("{}", e)
                    }
                }
            },
            Err(e) => eprintln!("{}", e)
        }
    }
}

