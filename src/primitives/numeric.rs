use std::ops::{Add, Sub, Mul, Div};

use utils::fraction;
use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use serr::{SErr, SResult};

pub fn calc(op_str: char, args: Args) -> SResult<SExpr> {
    let mut args_iter = args.eval()?
        .into_iter();

    let init = match op_str {
        '+' | '-' if args.len() == 1 => sint!(0),
        '*' | '/' if args.len() == 1 => sint!(1),
        _ => args_iter.next().ok_or_else(|| SErr::WrongArgCount(1,0))?
    };


    type I = fn(i64,i64)->i64;
    type Fl = fn(f64,f64)->f64;
    type Fr = fn(fraction::Fraction, fraction::Fraction)->fraction::Fraction;
    let (opi,opfl,opfr): (I, Fl, Fr) = match op_str {
        '+' => (Add::add,Add::add,Add::add),
        '-' => (Sub::sub, Sub::sub, Sub::sub),
        '*' => (Mul::mul, Mul::mul, Mul::mul),
        '/' => (Div::div, Div::div, Div::div),
        _   => bail!("Not an arithmetic op: {}", op_str)
    };

    use lexer::Token::*;
    use parser::SExpr::*;
    // Here we go, couldn't come up with something better
    let result = args_iter.fold(Ok(init), |acc, x|{
        let result = match (acc?, x) {
            (Atom(Integer(a)), Atom(Integer(b))) => {
                // Like it isnt ugly already
                if op_str == '/' && a % b != 0 {
                    Atom(Fraction(fraction::Fraction::new(a,b)))
                } else {
                    Atom(Integer(opi(a,b)))
                }
            },
            (Atom(Integer(a)), Atom(Float(b))) =>
                Atom(Float(opfl(a as f64, b))),
            (Atom(Float(a)), Atom(Integer(b))) =>
                Atom(Float(opfl(a,b as f64))),
            (Atom(Float(a)), Atom(Float(b))) =>
                Atom(Float(opfl(a,b))),
            (Atom(Fraction(a)), Atom(Fraction(b))) =>
                Atom(Fraction(opfr(a,b))),
            (Atom(Fraction(a)), Atom(Integer(b))) =>
                Atom(Fraction(opfr(a,From::from(b)))),
            (Atom(Integer(a)), Atom(Fraction(b))) =>
                Atom(Fraction(opfr(From::from(a), b))),
            (Atom(Fraction(a)), Atom(Float(b))) =>
                Atom(Float(opfl(a.into(),b))),
            (Atom(Float(a)), Atom(Fraction(b))) =>
                Atom(Float(opfl(a,b.into()))),
            (a,b) => bail!(TypeMismatch => "number", SExpr::List(vec![a, b]))
        };

        Ok(result)
    })?;

    // If it's an whole fraction, return it as int
    let fixed_result = if let Atom(Fraction(f)) = result {
        if f.is_int() {
            Atom(Integer(f.n))
        } else {
            result
        }
    } else {
        result
    };

    Ok(fixed_result)
}

pub fn exact_qm(args: Args) -> SResult<SExpr> {
    let result = args.eval()?
        .get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))
        .map(|x| match x {
            SExpr::Atom(Token::Integer(_)) |
                SExpr::Atom(Token::Fraction(_)) => Ok(true),
            SExpr::Atom(Token::Float(_)) => Ok(false),
            x => bail!(TypeMismatch => "number", x)
        })?;

    Ok(sbool!(result?))
}


pub fn inexact_qm(args: Args) -> SResult<SExpr> {
    Ok((!(exact_qm(args)?))?)
}

pub fn number_qm(args: Args) -> SResult<SExpr> {
    let is_number = args.evaled()?.own_one()?.is_numeric();
    Ok(sbool!(is_number))
}

pub fn remainder(args: Args) -> SResult<SExpr> {
    let (x, y) = args.evaled()?.own_two()?;

    Ok(sint!(x.as_int()? % y.as_int()?))
}
