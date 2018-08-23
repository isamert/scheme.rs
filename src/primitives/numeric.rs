use std::ops::{Add, Sub, Mul, Div};

use utils::fraction;
use utils::radix::Radix;
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


pub fn numerator(args: Args) -> SResult<SExpr> {
    let num = args.evaled()?.own_one()?;
    let result = match num {
        SExpr::Atom(Token::Integer(i)) => (fraction::Fraction::from(i).n),
        SExpr::Atom(Token::Float(i)) => (fraction::Fraction::from(i).n),
        SExpr::Atom(Token::Fraction(f)) => f.n,
        x => bail!(TypeMismatch => "number", x)
    };

    Ok(sint!(result))
}

pub fn denominator(args: Args) -> SResult<SExpr> {
    let num = args.evaled()?.own_one()?;
    let result = match num {
        SExpr::Atom(Token::Integer(i)) => (fraction::Fraction::from(i).d),
        SExpr::Atom(Token::Float(i)) => (fraction::Fraction::from(i).d),
        SExpr::Atom(Token::Fraction(f)) => f.d,
        x => bail!(TypeMismatch => "number", x)
    };

    Ok(sint!(result))
}

pub fn number_string(args: Args) -> SResult<SExpr> {
    if args.len() == 1 {
        let num = args.evaled()?.own_one()?;
        Ok(sstr!(num.to_string()))
    } else if args.len() == 2 {
        let (num, radix) = args.evaled()?.own_two()?;
        Ok(sstr!(Radix::new(num.into_float()?, radix.into_int()? as u32)?.to_string()))
    } else {
        bail!(WrongArgCount => 2 as usize, args.len())
    }
}

pub fn string_number(args: Args) -> SResult<SExpr> {
    if args.len() == 1 {
        use lexer::parse_number;
        let num_str = args.evaled()?.own_one()?.into_str()?;
        let num_token = parse_number(&num_str)
            .ok_or_else(|| SErr::new_generic(&format!("Can't parse as number: {}", num_str)))?;
        Ok(SExpr::Atom(num_token))
    } else if args.len() == 2 {
        bail!(Generic => "// FIXME: not implemented")
    } else {
        bail!(WrongArgCount => 2 as usize, args.len())
    }

}

#[macro_export]
macro_rules! call_float_fun(
    ($e: ident) => {
        |args| {
            let num = args.evaled()?.own_one()?;
            let result = num.into_float()?.$e();
            if result.trunc() == result {
                Ok((result as i64).into())
            } else {
                Ok(result.into())
            }
        }
    };
    ($e: ident, $e1: ident) => {
        |args| {
            use serr::SErr;
            let evaled = args.evaled()?;
            let result = match evaled.len() {
                1 => evaled.own_one()?.into_float()?.$e(),
                2 => {
                    let (f1, f2) = evaled.own_two()?;
                    f1.into_float()?.$e1(f2.into_float()?)
                },
                x => bail!(WrongArgCount => 2 as usize, x)
            };

            if result.trunc() == result {
                Ok((result as i64).into())
            } else {
                Ok(result.into())
            }
        }
    }
);
