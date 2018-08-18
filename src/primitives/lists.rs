use parser::SExpr;
use evaluator::Args;
use serr::{SErr, SResult};

pub fn list(args: Args) -> SResult<SExpr> {
    Ok(SExpr::List(args.eval()?))
}

pub fn cons(args: Args) -> SResult<SExpr> {
    let (x, xs, _rest) = args.evaled()?
        .own_two()?;

    let result = match xs {
        SExpr::List(mut xs) => {
            xs.insert(0, x);
            SExpr::List(xs)
        },
        SExpr::DottedList(mut xs, y) => {
            xs.insert(0, x);
            SExpr::DottedList(xs, y)
        },
        y => SExpr::DottedList(vec![x], Box::new(y))
    };

    Ok(result)
}

pub fn car(args: Args) -> SResult<SExpr> {
    let (xs, _rest) = args.evaled()?
        .own_one()?;

    let result = match xs {
        SExpr::List(ys) | SExpr::DottedList(ys, _) => ys.into_iter().next().unwrap(),
        x => bail!(UnexpectedForm => x)
    };

    Ok(result)
}

pub fn cdr(args: Args) -> SResult<SExpr> {
    let (xs, _rest) = args.evaled()?
        .own_one()?;

    let result = match xs {
        SExpr::List(ys) => SExpr::List(ys.into_iter().skip(1).collect()),
        SExpr::DottedList(ys, y) => {
            if ys.len() == 1 {
                *y
            } else {
                SExpr::DottedList(ys.into_iter().skip(1).collect(), y)
            }
        },
        x => bail!(UnexpectedForm => x)
    };

    Ok(result)
}
