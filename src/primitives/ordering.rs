use std::cmp::PartialOrd as po;
use std::cmp::PartialEq as pe;
use parser::SExpr;
use evaluator::Args;
use serr::SResult;

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
    Ok(SExpr::boolean(check(args.eval()?.as_slice(), op)))
}

fn check<I,F>(xs: &[I], op: F) -> bool
where I: PartialOrd,
      F: Fn(&I,&I) -> bool {
    match xs {
        [] | [_] => true,
        [x1, x2] => op(x1,x2),
        _ => {
            let x1 = &xs[0];
            let x2 = &xs[1];
            let rest = &xs[2..];

            op(x1,x2) && check(rest, op)
        }
    }
}
