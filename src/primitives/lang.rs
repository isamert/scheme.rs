use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use evaluator;
use evaluator::Extra;
use evaluator::ToArgs;
use procedure::ProcedureData;
use env::EnvRefT;

pub fn define(args: Args) -> SExpr {
    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.");

    let name = if let SExpr::Atom(Token::Symbol(name)) = name_expr {
        name
    } else {
        panic!("Expected an identifier, not an expr.")
    };

    let value = args.get(1)
        .expect("Expected an expression, found nothing.");

    let value_sexpr = value.eval(&args.env);

    args.env.insert(name.to_string(), value_sexpr.clone());

    value_sexpr
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

    println!("UNQUOTE: {}", arg);

    if level == 0 {
        arg.eval(&env)
    } else if level > 0 {
        let args = Args::new(vec![arg], Extra::QQLevel(level), &env);
        SExpr::unquote(eval_unquoted(args))
    } else {
        panic!("haydaaa")
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
/*
pub fn quasiquote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    let level = match args.extra {
        Extra::QQLevel(x) => x + 1,
        _ => 1
    };

    eval_unquote(
        Args::new(
            args.all().clone(),
            Extra::QQLevel(level),
            &args.env
        )
    )
}

pub fn unquote(args: Args) -> SExpr {
    let level = match args.extra {
        Extra::QQLevel(x) => x - 1,
        _ => panic!("`unquote` outside `quasiquote` is meaningless.")
    };

    if level == 0 {
        args.get(0)
            .expect("`unquote` needs an argument.")
            .eval(&args.env)
    } else {
        let arg = args.get(0)
            .expect("`unquote` needs an argument.")
            .clone();

        eval_unquote(Args::new(
            vec![arg],
            Extra::QQLevel(level),
            &args.env
        ))
    }
}


fn eval_unquote(args: Args) -> SExpr {
    let level = match args.extra {
        Extra::QQLevel(x) => x,
        _ => panic!("AAAAAAAAAAAAAAAAA")
    };

    let sexpr = args.get(0)
        .expect("Need arguments");

    match sexpr {
        SExpr::List(ref xs) => match xs[0] {
            SExpr::Atom(Token::Symbol(ref x)) => match x.as_str() {
                "unquote" => {
                    let args = Args::new(
                        xs[1..].to_vec(),
                        Extra::QQLevel(level),
                        &args.env
                    );
                    unquote(args)
                },
//                "unquote-splicing" => xs[1].eval(&args.env), // FIXME: fix this
                "quasiquote" => {
                    let args = Args::new(
                        xs[1..].to_vec(),
                        Extra::QQLevel(level),
                        &args.env
                    );
                    quasiquote(args)
                }
                _ => {
                    let mut items: Vec<SExpr> = xs.iter()
                        .skip(1)
                        .map(|x| eval_unquote(Args::new(vec![x.clone()], Extra::QQLevel(level), &args.env)))
                        .collect();

                    items.insert(0, xs[0].clone());
                    SExpr::List(items)
                }
            },
            SExpr::List(ref xs) => {
                eval_unquote(Args::new(
                    xs.clone(),
                    Extra::QQLevel(level),
                    &args.env
                ))
            },
            _ => sexpr.clone()
        }
        _ => sexpr.clone()
    }
}
*/
