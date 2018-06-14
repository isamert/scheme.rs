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
}

pub fn add(args: Args) -> SExpr {
    calc(op::add, args)
}

pub fn sub(args: Args) -> SExpr {
    calc(op::sub, args)
}

pub fn mult(args: Args) -> SExpr {
    calc(op::mult, args)
}

pub fn div(args: Args) -> SExpr {
    calc(op::div, args)
}

// TODO: Wrap in a helper module?
fn calc(op: (fn(f64,f64) -> f64), args: Args) -> SExpr {
    // FIXME: (- 5) should evaluate to -5
    let mut args_unwrapped = args
        .eval()
        .into_iter()
        .map(|x| match x {
            SExpr::Atom(Token::Integer(i)) => i as f64,
            SExpr::Atom(Token::Float(f)) => f,
            _ => panic!("Expected a number got {:#?}", x)
        });
    let init = args_unwrapped.next()
        .expect("Expected an argument, found none");
    let result = args_unwrapped.fold(init, |x, acc| op(x, acc));

    SExpr::Atom(Token::Float(result))
}


