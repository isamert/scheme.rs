use lexer::Token;
use parser::SExpr;
use evaluator::Args;

mod op {
    pub fn add(x: f64, y: f64) -> f64 {
        x + y
    }

    pub fn sub(x: f64, y: f64) -> f64 {
        x - y
    }

    pub fn mult(x: f64, y: f64) -> f64 {
        x * y
    }

    pub fn div(x: f64, y: f64) -> f64 {
        x / y
    }

    pub fn addi(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn subi(x: i64, y: i64) -> i64 {
        x - y
    }

    pub fn multi(x: i64, y: i64) -> i64 {
        x * y
    }

    pub fn divi(x: i64, y: i64) -> i64 {
        x / y
    }
}

pub fn calc(op_str: char, args: Args) -> SExpr {
    // FIXME: (- 5) should evaluate to -5
    let mut args_iter = args.eval()
        .into_iter();
    let init = args_iter.next()
        .expect("Expected an argument, found none");

    type F = fn(f64,f64)->f64;
    type I = fn(i64,i64)->i64;
    let (op, opi): (F,I) = match op_str {
        '+' => (op::add, op::addi),
        '-' => (op::sub, op::subi),
        '*' => (op::mult, op::multi),
        '/' => (op::div, op::divi),
        _   => panic!("Not an arithmetic op: {}", op_str)
    };

    use self::SExpr::Atom;
    use self::Token::{Integer, Float};
    args_iter.fold(init, |acc, x| match (acc, x) {
        (Atom(Integer(a)), Atom(Integer(b))) =>
            Atom(Integer(opi(a,b))),
        (Atom(Integer(a)), Atom(Float(b))) =>
            Atom(Float(op(a as f64, b))),
        (Atom(Float(a)), Atom(Integer(b))) =>
            Atom(Float(op(a,b as f64))),
        (Atom(Float(a)), Atom(Float(b))) =>
            Atom(Float(op(a,b))),
        (a,b) => panic!("At least one of these is not a number: {}, {}", a, b)
    })
}
