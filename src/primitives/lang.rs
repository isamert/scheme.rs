use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use evaluator::Extra;
use procedure::ProcedureData;
use env::EnvRefT;


fn zirt(args: Args) -> (String, SExpr) {
    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.");

    let id = match name_expr {
        SExpr::Atom(Token::Symbol(id)) => id,
        _ => panic!("Expected an identifier, not an expr.")
    };

    let value = args.get(1)
        .expect("Expected an expression, found nothing.");

    let value_sexpr = value.eval(&args.env);

    (id.clone(), value_sexpr)
}

pub fn define(args: Args) -> SExpr {
    let env = args.env();
    let (id, value) = zirt(args);
    env.define(id, value);

    SExpr::Unspecified
}

pub fn set(args: Args) -> SExpr {
    let env = args.env();
    let (id, value) = zirt(args);
    env.set(id, value);

    SExpr::Unspecified
}

pub fn lambda(args: Args) -> SExpr {
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

    ProcedureData::new(params, body, &args.env)
}


pub fn quote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    args.get(0)
        .unwrap()
        .clone()
}

pub fn quasiquote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    let level = match args.extra {
        Extra::QQLevel(x) => x + 1,
        _ => 1
    };

    let env = &args.env.clone();
    let args = Args::new(
        args.into_all(),
        Extra::QQLevel(level),
        &env
    );

    if level == 1 {
        eval_unquoted(args)
    } else if level > 1 {
        SExpr::quasiquote(vec!(eval_unquoted(args)))
    } else {
        panic!("haydaaaqq")
    }
}

pub fn unquote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    let level = match args.extra {
        Extra::QQLevel(x) => x - 1,
        _ => panic!("`unquote` is outside of `quasiquote`.")
    };

    let env = args.env.clone();
    let arg = args.into_all().pop().unwrap();

    if level == 0 {
        arg.eval(&env)
    } else if level > 0 {
        let args = Args::new(vec![arg], Extra::QQLevel(level), &env);
        SExpr::unquote(eval_unquoted(args))
    } else {
        panic!("Wrong call to `,`.")
    }
}

pub fn eval_unquoted(args: Args) -> SExpr {
    let arg = args.get(0)
        .expect("Expected argument found nothing.");

    let level = match args.extra {
        Extra::QQLevel(x) => x,
        _ => panic!("Not inside a `quasiquote`")
    };

    match arg {
        SExpr::List(xs) => match xs[0] {
            SExpr::Atom(Token::Symbol(ref x)) if x.as_str() == "unquote" => {
                unquote(Args::new(xs[1..].to_vec(), Extra::QQLevel(level), &args.env))
            },
            SExpr::Atom(Token::Symbol(ref x)) if x.as_str() == "quasiquote" => {
                quasiquote(Args::new(xs[1..].to_vec(), Extra::QQLevel(level), &args.env))
            },
            SExpr::List(ref xs2) => {
                SExpr::List(vec![eval_unquoted(Args::new(vec![SExpr::List(xs2.clone())], Extra::QQLevel(level), &args.env))])
            },
            _ => {
                let result = xs.iter()
                    .map(|x| eval_unquoted(Args::new(vec![x.clone()], Extra::QQLevel(level), &args.env)))
                    .collect();
                SExpr::List(result)
            }
        },
        x => x.clone()
    }
}
