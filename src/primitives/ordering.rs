use std::cmp::PartialOrd as po;
use std::cmp::PartialEq as pe;
use parser::SExpr;
use evaluator::Args;
use env::EnvRef;
use serr::{SErr, SResult};

pub fn lt(args: Args) -> SResult<SExpr> {
    compare(args, po::lt)
}

pub fn gt(args: Args) -> SResult<SExpr> {
    compare(args, po::gt)
}

pub fn lte(args: Args) -> SResult<SExpr> {
    compare(args, po::le)
}

pub fn gte(args: Args) -> SResult<SExpr> {
    compare(args, po::ge)
}

pub fn eq(args: Args) -> SResult<SExpr> {
    compare(args, pe::eq)
}

fn compare<F>(args: Args, op: F) -> SResult<SExpr>
where F: Fn(&SExpr,&SExpr) -> bool {
    Ok(sbool!(check(&args, op, &args.env)?))
}

fn check<F>(xs: &[SExpr], op: F, env: &EnvRef) -> SResult<bool>
where F: Fn(&SExpr,&SExpr) -> bool {
    match xs {
        [] | [_] => Ok(true),
        _ => {
            let x1 = xs[0].eval(env)?;
            let x2 = xs[1].eval(env)?;
            let rest = &xs[2..];
            if !(x1.is_numeric() && x2.is_numeric()) {
                bail!(TypeMismatch => "number", slist![x1, x2])
            }

            Ok(op(&x1, &x2) && check(rest, op, env)?)
        }
    }
}
