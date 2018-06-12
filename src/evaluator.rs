use std::rc::Rc;

use lexer::Token;
use parser::SExpr;
use env::EnvRef;
use closure::ClosureData;

pub fn eval(sexpr: &SExpr, env: EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.borrow_mut()
                .as_ref()
                .expect("Cannot find environment.")
                .get(x)
        },
        SExpr::Atom(x) => {
            SExpr::Atom(x.clone())
        },
        SExpr::Closure(_) => {
            panic!("YOU FUCKED UP")
        },
        SExpr::List(xs) => {
            let op = xs.get(0)
                .expect("Expected an operator, found nothing.");

            match op {
                SExpr::Atom(Token::Symbol(symbol)) =>
                    match symbol.trim() {
                        "define" => {
                            let name_expr = xs.get(1)
                                .expect("Expected an identifier, found nothing.");

                            let name = if let SExpr::Atom(Token::Symbol(name)) = name_expr {
                               name 
                            } else {
                                panic!("Expected an identifier, not an expr.")
                            };

                            let value = xs.get(2)
                                .expect("Expected an expression, found nothing.");

                            let value_sexpr = eval(value, Rc::clone(&env));

                            env.borrow_mut() // Mutable borrow RefCell
                                .as_mut()    // Get mutable reference to Env inside Option
                                .expect("Cannot find environment")
                                .insert(name.to_string(), value_sexpr.clone());

                            value_sexpr
                        },
                        "lambda" => {
                            let params_expr = xs.get(1)
                                .expect("Expected a list of parameters, found nothing.");

                            let params = if let SExpr::List(params) = params_expr {
                                params.iter()
                                    .map(|x| if let SExpr::Atom(Token::Symbol(symbol)) = x {
                                        symbol.clone()
                                    } else {
                                        panic!("Expected a parameter name, found this: {:#?}", x);
                                    })
                                    .collect::<Vec<String>>()
                            } else {
                                panic!("Expected an identifier, not an expr.")
                            };

                            let body = xs[2..].to_vec();

                            SExpr::Closure(ClosureData::new(params, body, env.clone()))
                        },
                        _ => {
                            // Skip the op name
                            let args = xs[1..].to_vec();
                            call_function(symbol, args, Rc::clone(&env))
                        },
                    },
                    x => {
                        // Trying to use something other than a symbol as function
                        panic!("Wrong type to apply: {:#?}", x);
                    }
                }
            }
        }
    }


// TODO: ask for &str instead of &SExpr
fn call_function(op: &str, args: Vec<SExpr>, env: EnvRef) -> SExpr {
    let evaluate_args = |args: Vec<SExpr>| {
        args.into_iter()
            .map(|x| eval(&x, Rc::clone(&env)))
            .collect::<Vec<SExpr>>()
    };

    match op.as_ref() {
        "+" | "-" | "*" | "/" => {
            do_arithmetic(op, evaluate_args(args))
        },
        "<" | ">" | "<=" | ">=" => {
            do_compare(op, evaluate_args(args))
        },
        "if" => {
            let condition = args.get(0)
                .expect("Expected a boolean or expression, found nothing");
            let on_true = args.get(1)
                .expect("Expected an expression, found nothing.");
            let on_false = args.get(2)
                .expect("Expected an expression, found nothing.");

            if to_bool(eval(condition, Rc::clone(&env))) {
                eval(on_true, Rc::clone(&env))
            } else {
                eval(on_false, Rc::clone(&env))
            }
        },
        _ => { // Try to call a closure
            let closure = env.borrow()
                .as_ref()
                .expect("Cannot find environment")
                .get(op);
            if let SExpr::Closure(c) = closure {
                c.run(evaluate_args(args))
            } else {
                panic!("Not a type to apply: {:#?}", closure)
            }
        }
    }
}

fn do_arithmetic(op_str: &str, args: Vec<SExpr>) -> SExpr {
    let (op, init): (fn(f64,f64) -> f64, f64) = match op_str {
        "+" => (add, 0.0),
        "-" => (sub, 0.0),
        "*" => (mult, 1.0),
        "/" => (div, 1.0),
        _ => panic!("Expected an arithmetic operation, got {}", op_str)
    };

    // FIXME: dividing does not work as expected because of the initil value
    let result = args.into_iter()
        .map(|x| match x {
            SExpr::Atom(Token::Integer(i)) => i as f64,
            SExpr::Atom(Token::Float(f)) => f,
            _ => panic!("Expected a number got {:#?}", x)
        })
        .fold(init, |x, acc| op(acc, x));

    SExpr::Atom(Token::Float(result))
}


fn do_compare(op_str: &str, args: Vec<SExpr>) -> SExpr {
    let result = if let (SExpr::Atom(arg1), SExpr::Atom(arg2)) = (&args[0], &args[1]) {
        match op_str {
            "<"  => arg1 < arg2,
            ">"  => arg1 > arg2,
            "<=" => arg1 <= arg2,
            ">=" => arg1 >= arg2,
            _ => panic!("Expected an ordering operation, got {}", op_str)
        }
    } else {
        panic!("Expected an atom, found something else.");
    };

    SExpr::Atom(Token::Boolean(result))
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn sub(x: f64, y: f64) -> f64 {
    x - y
}

fn mult(x: f64, y: f64) -> f64 {
    x * y
}

fn div(x: f64, y: f64) -> f64 {
    x / y
}

fn to_bool(x: SExpr) -> bool {
    // Anything other than #f is treated as true.
    match x {
        SExpr::Atom(Token::Boolean(x)) => x,
        _ => true
    }
}
