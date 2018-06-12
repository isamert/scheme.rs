#[macro_use]
mod util;
mod env;
mod lexer;
mod parser;
mod closure;
mod evaluator;
mod primitives;

use std::rc::Rc;
use std::cell::RefCell;

use parser::SExpr;
use env::Env;

fn main() {
    let tokens = lexer::tokenize("
(+ (- 3 2) (/ 10 2) (* 3 4 5))
(< 3 5)
    ");

    //println!("{:#?}", tokens);
    let sexprs = parser::parse(tokens);
    let global_env = Env::with_values(Env::null(), primitives::env());
    let global_env_ref = global_env.to_ref();

    for (i, sexpr) in sexprs.iter().enumerate() {
        //println!("{:#?}", sexpr);
        let evaluated = evaluator::eval(&sexpr, Rc::clone(&global_env_ref));
        if let SExpr::Closure(_) = evaluated {
        } else {
            println!("${}: {:#?}", i, evaluated);
        }
    }
}

