#[macro_use]
mod util;
mod env;
mod lexer;
mod parser;
mod procedure;
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
(quote 1)
(define a 
  (lambda (a1 a2)
    (define b (lambda (b1) 
      (define c (lambda (c1) 
        (if (> b1 c1) b1 c1)))
      (* (c 12) a2)))
    (+ (b 4) a2)))

(a 2 3)
    ");

    //println!("{:#?}", tokens);
    let sexprs = parser::parse(tokens);
    let global_env = Env::with_values(Env::null(), primitives::env());
    let global_env_ref = global_env.to_ref();

    for (i, sexpr) in sexprs.iter().enumerate() {
        //println!("{:#?}", sexpr);
        let evaluated = evaluator::eval(&sexpr, Rc::clone(&global_env_ref));
        if let SExpr::Procedure(_) = evaluated {
            println!("${}: <procedure>", i);
        } else {
            println!("${}: {:#?}", i, evaluated);
        }
    }
}

