use parser::SExpr;
use evaluator::Args;
use serr::{SErr, SResult};

pub fn boolean_qm(args: Args) -> SResult<SExpr> {
    let is_bool = args.evaled()?
        .own_one()?
        .is_boolean();

    Ok(sbool!(is_bool))
}

// boolean=?
pub fn boolean_eq_qm(args: Args) -> SResult<SExpr> {
    let env = args.env();
    let mut iter = args.into_iter();

    let control = iter.next()
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval(&env)?;

    if !control.is_boolean() {
        bail!(TypeMismatch => "boolean", control)
    }

    for b in iter {
        let b_evaled = b.eval(&env)?;
        let b_is_bool = b.is_boolean();

        if !b_is_bool {
            bail!(TypeMismatch => "boolean", b)
        } else if b_evaled != control {
            return Ok(sbool!(false))
        }
    }

    Ok(sbool!(true))
}
