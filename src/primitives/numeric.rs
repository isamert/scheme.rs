use std::ops::{Add, Sub, Mul, Div};

use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use util;

pub fn calc(op_str: char, args: Args) -> SExpr {
    let mut args_iter = args.eval()
        .into_iter();
    let init = match op_str {
        '+' | '-' if args.len() == 1 => SExpr::integer(0),
        '*' | '/' if args.len() == 1 => SExpr::integer(1),
        _ => args_iter.next().expect("Expected an argument, found none")
    };


    type I = fn(i64,i64)->i64;
    type Fl = fn(f64,f64)->f64;
    type Fr = fn(util::Fraction, util::Fraction)->util::Fraction;
    let (opi,opfl,opfr): (I, Fl, Fr) = match op_str {
        '+' => (Add::add,Add::add,Add::add),
        '-' => (Sub::sub, Sub::sub, Sub::sub),
        '*' => (Mul::mul, Mul::mul, Mul::mul),
        '/' => (Div::div, Div::div, Div::div),
        _   => panic!("Not an arithmetic op: {}", op_str)
    };

    use lexer::Token::*;
    use parser::SExpr::*;
    // Here we go, couldn't come up with something better
    let result = args_iter.fold(init, |acc, x| match (acc, x) {
        (Atom(Integer(a)), Atom(Integer(b))) => {
            // Like it isnt ugly already
            if op_str == '/' && a % b != 0 { 
                Atom(Fraction(util::Fraction::new(a,b)))
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
        (a,b) => panic!("At least one of these is not a number: {}, {}", a, b)
    });

    // If it's an whole fraction, return it as int
    if let Atom(Fraction(f)) = result {
        if f.is_int() {
            Atom(Integer(f.n))
        } else {
            result
        }
    } else {
        result
    }
}

pub fn exact(args: Args) -> SExpr {
    let result = args.eval()
        .iter()
        .all(|x| match x {
            SExpr::Atom(Token::Integer(_)) | 
                SExpr::Atom(Token::Fraction(_)) => true,
            SExpr::Atom(Token::Float(_)) => false,
            _ => panic!("Wrong type of argument while using exact?")
        });

    SExpr::boolean(result)
}


pub fn inexact(args: Args) -> SExpr {
    !exact(args)
}
