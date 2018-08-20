use parser::SExpr;
use evaluator::Args;
use serr::{SErr, SResult};

pub fn list(args: Args) -> SResult<SExpr> {
    Ok(SExpr::List(args.eval()?))
}

pub fn cons(args: Args) -> SResult<SExpr> {
    let (x, xs) = args.evaled()?
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
    let xs = args.evaled()?
        .own_one()?;

    match xs {
        SExpr::List(ys) | SExpr::DottedList(ys, _) => {
            ys.into_iter()
                .next()
                .ok_or_else(|| SErr::new_generic("car: empty list"))
        },
        x => bail!(UnexpectedForm => x)
    }
}

pub fn cdr(args: Args) -> SResult<SExpr> {
    let xs = args.evaled()?
        .own_one()?;

    let result = match xs {
        SExpr::List(ys) => {
            let mut iter = ys.into_iter();
            iter.next() // Skip 1
                .ok_or_else(|| SErr::new_generic("cdr: empty list"))?;

            SExpr::List(iter.collect())
        },
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

pub fn append(args: Args) -> SResult<SExpr> {
    let len = args.len();
    if len == 1 {
        return args[0].eval(&args.env)
    }

    let (xs, rest) = args.evaled()?
        .own_one_rest()?;
    let mut list = xs.into_list()?;
    let iter = rest.into_iter();

    for (i, expr) in iter.enumerate() {
        // list is the first element, and i starts from 0, so -2
        if i == len - 2 {
            match expr {
                SExpr::List(mut xs) => {
                    list.append(&mut xs);
                    return Ok(SExpr::List(list))
                },
                SExpr::DottedList(mut xs, y) => {
                    list.append(&mut xs);
                    return Ok(SExpr::dottedlist(list, *y))
                },
                x => return Ok(SExpr::dottedlist(list, x))
            }
        } else {
            list.append(&mut expr.into_list()?);
        }
    }

    // Just for satisfying compiler
    Ok(SExpr::Unspecified)
}

pub fn null_qm(args: Args) -> SResult<SExpr> {
    let x = args.evaled()?
        .own_one()?;

    let result = match x {
        SExpr::List(ref ys) if ys.is_empty() => true,
        _ => false
    };

    Ok(SExpr::boolean(result))
}

pub fn pair_qm(args: Args) -> SResult<SExpr> {
    let result = args.evaled()?
        .own_one()?
        .is_pair();

    Ok(SExpr::boolean(result))
}

pub fn list_qm(args: Args) -> SResult<SExpr> {
    let result = args.evaled()?
        .own_one()?
        .is_proper_list();

    Ok(SExpr::boolean(result))
}
