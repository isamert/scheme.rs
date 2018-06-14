use lexer::Token;
use parser::SExpr;
use evaluator;
use evaluator::Args;
use procedure::ProcedureData;
use env::EnvValues;
use env::EnvRef;
use env::EnvRefT;

pub fn env() -> EnvValues {
    environment! {
        "+"  => ProcedureData::new_primitive(|args| do_arithmetic(add, args)),
        "-"  => ProcedureData::new_primitive(|args| do_arithmetic(sub, args)),
        "*"  => ProcedureData::new_primitive(|args| do_arithmetic(mult, args)),
        "/"  => ProcedureData::new_primitive(|args| do_arithmetic(div, args)),
        "<"  => ProcedureData::new_primitive(|args| do_compare("<", args)),
        ">"  => ProcedureData::new_primitive(|args| do_compare(">", args)),
        "<=" => ProcedureData::new_primitive(|args| do_compare("<=", args)),
        ">=" => ProcedureData::new_primitive(|args| do_compare(">=", args)),
        "define" => ProcedureData::new_primitive(do_define),
        "lambda" => ProcedureData::new_primitive(do_lambda),
        "if"     => ProcedureData::new_primitive(do_if),
        "quote"  => ProcedureData::new_primitive(do_quote),
        "list"   => ProcedureData::new_primitive(do_list),
        "car"    => ProcedureData::new_primitive(do_car),
        "cdr"    => ProcedureData::new_primitive(do_cdr)
    }
}


fn do_define(args: Args) -> SExpr {
    let env = args.env();

    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.");

    let name = if let SExpr::Atom(Token::Symbol(name)) = name_expr {
        name 
    } else {
        panic!("Expected an identifier, not an expr.")
    };

    let value = args.get(1)
        .expect("Expected an expression, found nothing.");

    let value_sexpr = evaluator::eval(value, env.clone_ref());

    env.insert(name.to_string(), value_sexpr.clone());

    value_sexpr
}

fn do_lambda(args: Args) -> SExpr {
    let params_expr = args.get(0)
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

    let body = args.all()[1..].to_vec();

    ProcedureData::new(params, body, args.env())
}

fn do_if(args: Args) -> SExpr {
    let condition = args.get(0)
        .expect("Expected a boolean or expression, found nothing");
    let on_true = args.get(1)
        .expect("Expected an expression, found nothing.");
    let on_false = args.get(2)
        .expect("Expected an expression, found nothing.");

    let env = args.env();
    if to_bool(evaluator::eval(condition, env.clone_ref())) {
        evaluator::eval(on_true, env.clone_ref())
    } else {
        evaluator::eval(on_false, env.clone_ref())
    }
}

fn do_quote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    args.get(0)
        .unwrap()
        .clone()
}

fn do_list(args: Args) -> SExpr {
    let list : Vec<SExpr> = args.all()
        .iter()
        .map(|x| match x {
            x@SExpr::List(_) => evaluator::eval(x, args.env()),
            x              => x.clone()
        })
        .collect();

    SExpr::List(list)
}

fn do_car(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong argument count to car.");
    }

    let list = &args.eval()[0];
    match list {
        SExpr::List(x) => x[0].clone(),
        _              => panic!("Wrong type of argument to car.")
    }
}

fn do_cdr(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong argument count to cdr.");
    }

    let list = &args.eval()[0];
    match list {
        SExpr::List(x) => SExpr::List(x[1..].to_vec()),
        _              => panic!("Wrong type of argument to cdr.")
    }
}

fn do_arithmetic(op: (fn(f64,f64) -> f64), args: Args) -> SExpr {
    // FIXME: (- 5) should evaluate to -5
    let mut args_unwrapped = args
        .eval()
        .into_iter()
        .map(|x| match x {
            SExpr::Atom(Token::Integer(i)) => i as f64,
            SExpr::Atom(Token::Float(f)) => f,
            _ => panic!("Expected a number got {:#?}", x)
        });
    let init = args_unwrapped.next()
        .expect("Expected an argument, found none");
    let result = args_unwrapped.fold(init, |x, acc| op(x, acc));

    SExpr::Atom(Token::Float(result))
}

fn do_compare(op_str: &str, args: Args) -> SExpr {
    let evaled_args = args.eval();
    let result = if let (SExpr::Atom(arg1), SExpr::Atom(arg2)) = (&evaled_args[0], &evaled_args[1]) {
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
