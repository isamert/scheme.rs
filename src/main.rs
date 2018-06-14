#[macro_use]
mod util;
mod env;
mod lexer;
mod parser;
mod procedure;
mod evaluator;
mod primitives;

use parser::SExpr;
use env::Env;
use env::EnvRefT;

fn main() {
    let tokens = lexer::tokenize("
(define a (list a b c d e f 1 2 3 4))
(car a)
(cdr a)
(car (cdr (cdr (cdr a))))
    ");

    //println!("{:#?}", tokens);
    let sexprs = parser::parse(tokens);
    let globalenv = Env::with_values(Env::null(), primitives::env());
    let globalenv_ref = globalenv.to_ref();

    for (i, sexpr) in sexprs.iter().enumerate() {
        //println!("{:#?}", sexpr);
        let evaluated = evaluator::eval(&sexpr, globalenv_ref.clone_ref());
        if let SExpr::Procedure(_) = evaluated {
            println!("${}: <procedure>", i);
        } else {
            println!("${}: {:#?}", i, evaluated);
        }
    }
}

