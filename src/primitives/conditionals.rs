use parser::SExpr;
use parser::SExprs;
use evaluator::Args;
use evaluator::ToArgs;
use serr::{SErr, SResult};


pub fn cond(args: Args) -> SResult<SExpr> {
    let clauses = args.all()
        .iter()
        .map(|x| {
            if let SExpr::List(clause) = x {
                let mut current = 0;
                let test = clause.get(current)
                    .ok_or_else(|| SErr::new_unexpected_form(x))?;

                current += 1;
                if clause.len() == 3 {
                    // Consume `=>`
                    // FIXME: check if `clause.get(current)` is => otherwise panic!
                    current += 1;
                }

                let expr = clause.get(current)
                    .ok_or_else(|| SErr::new_unexpected_form(x))?;

                Ok((test, expr))
            } else {
                bail!(TypeMismatch => "list of clauses", x)
            }
        })
        .collect::<SResult<Vec<_>>>()?;

    let mut else_clause: Option<SExpr> = None;
    for (test, expr) in clauses {
        if test.is_symbol("else") {
            if else_clause.is_some() { bail!(UnexpectedForm => test) }
            else_clause = Some(expr.clone());
        } else if test.eval(&args.env)?.to_bool() {
            return expr.eval(&args.env)
        }
    }

    if else_clause.is_some() {
        else_clause.unwrap()
            .eval(&args.env)
    } else {
        Ok(SExpr::Unspecified)
    }
}

pub fn case(args: Args) -> SResult<SExpr> {
    let test = args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1,0))?;

    let args_vec: SExprs = args.all()
        .iter()
        .skip(1)
        .map(|clause| {
            if let SExpr::List(xs) = clause {
                let test = SExpr::List(vec![SExpr::symbol("eqv?"), xs[0].clone(), test.clone()]);
                Ok(SExpr::List(vec![test, xs[1].clone()]))
            } else {
                bail!(UnexpectedForm => clause)
            }
        })
        .collect::<SResult<_>>()?;

    cond(args_vec.to_args(&args.env))
}

pub fn or(args: Args) -> SResult<SExpr> {
    for expr in args.all() {
        if expr.eval(&args.env)?.to_bool() { return Ok(SExpr::boolean(true)) }
    }

    Ok(SExpr::boolean(false))
}

pub fn and(args: Args) -> SResult<SExpr> {
    for expr in args.all() {
        if !expr.eval(&args.env)?.to_bool() { return Ok(SExpr::boolean(false)) }
    }

    Ok(SExpr::boolean(true))
}
