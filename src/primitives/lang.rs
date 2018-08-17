use lexer::Token;
use parser::SExpr;
use parser::SExprs;
use evaluator::Args;
use evaluator::Extra;
use procedure::ProcedureData;
use env::EnvRefT;
use env::EnvRef;
use env::Env;
use serr::{SErr, SResult};

pub fn define(args: Args) -> SResult<SExpr> {
    env_add(EnvAddType::Define, args)
}

pub fn set(args: Args) -> SResult<SExpr> {
    env_add(EnvAddType::Set, args)
}

pub fn lambda(args: Args) -> SResult<SExpr> {
    let env = args.env();
    let (params, body) = args.into_split()?;
    ProcedureData::new_compound(params, body, &env)
}

pub fn let_(args: Args) -> SResult<SExpr> {
    let_generic(args, |expr, _, parent_env| expr.eval(parent_env))
}

pub fn let_star(args: Args) -> SResult<SExpr> {
    let_generic(args, |expr, env, _| expr.eval(env))
}

pub fn let_rec(args: Args) -> SResult<SExpr> {
    let_generic(args, |expr, _, _| Ok(SExpr::lazy(expr.clone())))
}

pub fn quote(args: Args) -> SResult<SExpr> {
    if args.len() != 1 {
        bail!(WrongArgCount => 1 as usize, args.len())
    }

    Ok(args[0].clone())
}

pub fn quasiquote(args: Args) -> SResult<SExpr> {
    if args.len() != 1 {
        bail!(WrongArgCount => 1 as usize, args.len())
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
        Ok(SExpr::quasiquote(vec![eval_unquoted(args)?]))
    } else {
        bail!(Generic -> "Wrong usage of quasiquote")
    }
}

pub fn unquote(args: Args) -> SResult<SExpr> {
    if args.len() != 1 {
        bail!(WrongArgCount => 1 as usize, args.len())
    }

    let level = match args.extra {
        Extra::QQLevel(x) => x - 1,
        _ => bail!(Generic -> "Usage of unquote outside of quasiquote")
    };

    let env = args.env();
    let arg = args.into_all().pop().unwrap();

    if level == 0 {
        arg.eval(&env)
    } else if level > 0 {
        let args = Args::new_with_extra(vec![arg], Extra::QQLevel(level), &env);
        Ok(SExpr::unquote(eval_unquoted(args)?))
    } else {
        bail!(Generic -> "Wrong usage of unquote")
    }
}

pub fn eval_unquoted(args: Args) -> SResult<SExpr> {
    let arg = args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1,0))?;

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
                Ok(SExpr::List(vec![eval_unquoted(Args::new_with_extra(vec![SExpr::List(xs2.clone())], Extra::QQLevel(level), &args.env))?]))
            },
            _ => {
                let result = xs.iter()
                    .map(|x| eval_unquoted(Args::new_with_extra(vec![x.clone()], Extra::QQLevel(level), &args.env)))
                    .collect::<SResult<_>>();
                Ok(SExpr::List(result?))
            }
        },
        x => Ok(x.clone())
    }
}

//
// Helpers
//
#[derive(Clone, Copy)]
enum EnvAddType {
    Define,
    Set
}

fn env_add(t: EnvAddType, args: Args) -> SResult<SExpr> {
    let env = args.env();
    let name_expr = args.get(0)
        .ok_or_else(|| SErr::new_id_not_found("nothing"))?
        .clone();

    let (id, value) = match name_expr {
        SExpr::Atom(Token::Symbol(id)) => {
            let value = args.get(1)
                .ok_or_else(|| SErr::new_expr_not_found("nothing"))?;

            let value_sexpr = value.eval(&args.env)?;

            (id.clone(), value_sexpr)
        },
        SExpr::List(_) => {
            let (header, body) = args.into_split()?;
            let (id, params) = header.into_split()?;

            (id.into_symbol()?, ProcedureData::new_compound(SExpr::List(params), body, &env)?)
        },
        SExpr::DottedList(xs,y) => {
            let mut iter = xs.into_iter();
            let id = iter.next()
                .ok_or_else(|| SErr::new_generic("Expected an identifier, found nothing."))?;
            let head = iter.take_while(|_| true).collect::<SExprs>();
            let (_, body) = args.into_split()?;

            let arg_list = match head.len() {
                // (define (x . y) ...)
                0 => *y,
                // (define (x y ... . z) ...)
                _ => SExpr::DottedList(head, y)
            };

            (id.into_symbol()?, ProcedureData::new_compound(arg_list, body, &env)?)
        },
        _ => panic!("Expected an identifier, not an expr.")
    };


    match t {
        EnvAddType::Define => {
            env.define(id.clone(), value);
            Ok(SExpr::Unspecified)
        },
        EnvAddType::Set => {
            env.set(id.clone(), value)
        }
    }
}

pub fn let_generic<F>(args: Args, mut eval_expr: F) -> SResult<SExpr>
where F: (FnMut(&SExpr,/*env:*/ &EnvRef,/*parent_env:*/&EnvRef) -> SResult<SExpr>) {
    let parent_env = args.env();
    let (bindings, body) = args.into_split()
        .expect("Expected a list of bindings and body, found something else.");

    let env = Env::new(parent_env.clone())
        .into_ref();
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

        env.define(id, eval_expr(expr, &env, &parent_env)?);
    }

    let mut result = None;
    for expr in body {
        result = Some(expr.eval(&env));
    }

    result
        .expect("Let body is empty")
}
