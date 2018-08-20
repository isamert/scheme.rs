use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use serr::SResult;

pub fn eq_qm(args: Args) -> SResult<SExpr> {
    eqv_qm(args)
}

pub fn eqv_qm(args: Args) -> SResult<SExpr> {
    equality(args, |args| {
        let evaled = args.eval()?;
        let result = match (&evaled[0], &evaled[1]) {
            (SExpr::Atom(x), SExpr::Atom(y)) => x == y,
            (SExpr::List(x), SExpr::List(y)) => x.is_empty() && y.is_empty(),
            (SExpr::Vector(x), SExpr::Vector(y)) => x.is_empty() && y.is_empty(),
            (_,_) => false
        };

        Ok(result)
    })
}

pub fn equal_qm(args: Args) -> SResult<SExpr> {
    equality(args, |args| {
        let evaled = args.eval()?;
        let obj1 = &evaled[0];
        let obj2 = &evaled[1];

        Ok(obj1 == obj2)
    })
}

fn equality<F>(args: Args, mut non_atom: F) -> SResult<SExpr>
where F: (FnMut(&Args) -> SResult<bool>) {
    if args.len() < 2 {
        return Ok(SExpr::boolean(true));
    }

    let result = match (&args[0], &args[1]) {
        (x@SExpr::Atom(Token::Symbol(_)), y@SExpr::Atom(Token::Symbol(_))) => {
            x.eval_ref(&args.env, |x| {
                y.eval_ref(&args.env, |y| Ok(x == y))
            })?
        },
        (SExpr::Atom(x), SExpr::Atom(y)) => x == y,
        _ => {
            non_atom(&args)?
        }
    };

    Ok(SExpr::boolean(result))
}
