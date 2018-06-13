use lexer::Token;
use parser::SExpr;
use env::EnvRef;
use procedure::ProcedureData;
use env::EnvRefT;

pub fn eval(sexpr: &SExpr, env: EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.borrow_mut()
                .get(x)
        },
        SExpr::Atom(x) => {
            SExpr::Atom(x.clone())
        },
        SExpr::Procedure(_) => {
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

                            let value_sexpr = eval(value, env.clone_ref());

                            env.borrow_mut() // Mutable borrow RefCell
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

                            ProcedureData::new(params, body, env.clone())
                        },
                        _ => {
                            // Skip the op name
                            let args = xs[1..].to_vec();
                            call_function(symbol, args, env.clone_ref())
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


fn call_function(op: &str, args: Vec<SExpr>, env: EnvRef) -> SExpr {
    let evaluate_args = |args: Vec<SExpr>| {
        args.into_iter()
            .map(|x| eval(&x, env.clone_ref()))
            .collect::<Vec<SExpr>>()
    };

    match op.as_ref() {
        "if" => {
            let condition = args.get(0)
                .expect("Expected a boolean or expression, found nothing");
            let on_true = args.get(1)
                .expect("Expected an expression, found nothing.");
            let on_false = args.get(2)
                .expect("Expected an expression, found nothing.");

            if to_bool(eval(condition, env.clone_ref())) {
                eval(on_true, env.clone_ref())
            } else {
                eval(on_false, env.clone_ref())
            }
        },
        "quote" => {
            if args.len() != 1 {
                panic!("Wrong number of arguments while using `quote`.");
            }

            args.get(0)
                .unwrap()
                .clone()
        },
        _ => { // Try to call a procedure
            let procedure = env.borrow()
                .get(op);
            if let SExpr::Procedure(c) = procedure {
                c.run(evaluate_args(args))
            } else {
                panic!("Not a type to apply: {:#?}", procedure)
            }
        }
    }
}



fn to_bool(x: SExpr) -> bool {
    // Anything other than #f is treated as true.
    match x {
        SExpr::Atom(Token::Boolean(x)) => x,
        _ => true
    }
}
