use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use evaluator::Extra;
use procedure::ProcedureData;
use env::EnvRefT;
use env::EnvRef;
use env::Env;

enum EnvAddType {
    Define,
    Set
}

fn env_add(t: EnvAddType, args: Args) -> SExpr {
    let env = args.env();
    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.")
        .clone();

    let (id, value) = match name_expr {
        SExpr::Atom(Token::Symbol(id)) => {
            let value = args.get(1)
                .expect("Expected an expression, found nothing.");

            let value_sexpr = value.eval(&args.env);

            (id.clone(), value_sexpr)
        },
        SExpr::List(_) => {
            let (header, body) = args.into_split()
                .expect("");

            let (id, param_list) = header
                .into_split()
                .expect("");

            let params = param_list
                .into_iter()
                .map(|x|
                     x.into_symbol()
                     .expect("Expected a parameter name, found this: {:#?}"))
                .collect();

            (id.into_symbol().unwrap(), ProcedureData::new(params, body, &env))
        },
        _ => panic!("Expected an identifier, not an expr.")
    };


    match t {
        EnvAddType::Define => env.define(id.clone(), value),
        EnvAddType::Set    => env.set(id.clone(), value)
    }
    SExpr::Unspecified
}

pub fn define(args: Args) -> SExpr {
    env_add(EnvAddType::Define, args)
}

pub fn set(args: Args) -> SExpr {
    env_add(EnvAddType::Set, args)
}

pub fn lambda(args: Args) -> SExpr {
    let env = args.env();
    let (params_list, body) = args.into_split()
        .expect("Expected a parameter list and function body, found something else");

    let params = params_list
        .into_list()
        .expect("Expected a parameter list, found something else.")
        .into_iter()
        .map(|x|
            x.into_symbol()
                .expect("Expected a parameter name, found this: {:#?}")) // FIXME: can't display what's found.
        .collect::<Vec<String>>();

    ProcedureData::new(params, body, &env)
}

pub fn let_generic<F>(args: Args, mut eval_expr: F) -> SExpr
where F: (FnMut(&SExpr,/*env:*/ &EnvRef,/*parent_env:*/&EnvRef) -> SExpr) {
    let parent_env = args.env();
    let (bindings, body) = args.into_split()
        .expect("Expected a list of bindings and body, found something else.");

    let env = Env::new(parent_env.clone())
        .to_ref();
    let bindings_list = bindings.into_list()
        .expect("Expected a list of bindings, found something else.");

    for x in bindings_list {
        let bind = x.into_list()
            .expect("Expected a id-expr pair, found something else.");

        let id = bind.get(0)
            .expect("Expected an identifier, found nothing")
            .clone()
            .into_symbol()
            .expect("Identifier must be a symbol.");

        let expr = bind.get(1)
            .expect("Expected an expression, found nothing");

        env.define(id, eval_expr(expr, &env, &parent_env));
    }

    let mut result = None;
    for expr in body {
        result = Some(expr.eval(&env));
    }

    return result
        .expect("Let body is empty");
}

pub fn let_(args: Args) -> SExpr {
    let_generic(args, |expr, _, parent_env| expr.eval(parent_env))
}

pub fn let_star(args: Args) -> SExpr {
    let_generic(args, |expr, env, _| expr.eval(env))
}

pub fn let_rec(args: Args) -> SExpr {
    let_generic(args, |expr, _, _| SExpr::lazy(expr.clone()))
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

    let env = &args.env();
    let args = Args::new_with_extra(
        args.into_all(),
        Extra::QQLevel(level),
        &env
    );

    if level == 1 {
        eval_unquoted(args)
    } else if level > 1 {
        SExpr::quasiquote(vec!(eval_unquoted(args)))
    } else {
        panic!("Wrong call to ```.")
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

    let env = args.env();
    let arg = args.into_all().pop().unwrap();

    if level == 0 {
        arg.eval(&env)
    } else if level > 0 {
        let args = Args::new_with_extra(vec![arg], Extra::QQLevel(level), &env);
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
                unquote(Args::new_with_extra(xs[1..].to_vec(), Extra::QQLevel(level), &args.env))
            },
            SExpr::Atom(Token::Symbol(ref x)) if x.as_str() == "quasiquote" => {
                quasiquote(Args::new_with_extra(xs[1..].to_vec(), Extra::QQLevel(level), &args.env))
            },
            SExpr::List(ref xs2) => {
                SExpr::List(vec![eval_unquoted(Args::new_with_extra(vec![SExpr::List(xs2.clone())], Extra::QQLevel(level), &args.env))])
            },
            _ => {
                let result = xs.iter()
                    .map(|x| eval_unquoted(Args::new_with_extra(vec![x.clone()], Extra::QQLevel(level), &args.env)))
                    .collect();
                SExpr::List(result)
            }
        },
        x => x.clone()
    }
}
