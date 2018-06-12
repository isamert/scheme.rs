mod env;
mod util;
mod lexer;
mod parser;
mod closure;
mod evaluator;

use std::rc::Rc;
use std::cell::RefCell;

use parser::SExpr;
use env::Env;

fn main() {
    //let tokens = tokenize("
    //(define fizzbuzz
      //(lambda (n)
        //(cond ((= (modulo n 15) 0) (display \"FizzBuzz\") (newline))
              //((= (modulo n  3) 0) (display \"Fizz\")     (newline))
              //((= (modulo n  5) 0) (display \"Buzz\")     (newline))
              //(else (display n) (newline)))))"
    //);

    let tokens = lexer::tokenize("
(define plus 
  (lambda (x y) (+ x y)))

(define for (lambda (i j f) (if (>= i j) (+ 0 0) ((f) (for (+ i 1) j f)))))

(for 1 5 (lambda () (+ 1 1)))
");
    println!("{:#?}", tokens);
    let sexprs = parser::parse(tokens);
    let global_env = Env::new(Rc::new(RefCell::new(None)));
    let global_env_ref = Rc::new(RefCell::new(Some(global_env)));

    for (i, sexpr) in sexprs.iter().enumerate() {
        //println!("{:#?}", sexpr);
        let evaluated = evaluator::eval(&sexpr, Rc::clone(&global_env_ref));
        if let SExpr::Closure(_) = evaluated {
        } else {
            println!("${}: {:#?}", i, evaluated);
        }
    }
}

