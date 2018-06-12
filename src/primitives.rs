use lexer::Token;
use parser::SExpr;
use closure::ClosureData;
use env::EnvValues;

pub fn env() -> EnvValues {
    hashmap!{
        "+".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_arithmetic(add, args))),
        "-".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_arithmetic(sub, args))),
        "*".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_arithmetic(mult, args))),
        "/".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_arithmetic(div, args))),
        "<".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_compare("<", args))),
        ">".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_compare(">", args))),
        "<=".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_compare("<=", args))),
        ">=".to_string() =>
            SExpr::Closure(ClosureData::new_primitive(|args| do_compare(">=", args)))
    }
}

fn do_arithmetic(op: (fn(f64,f64) -> f64), args: Vec<SExpr>) -> SExpr {
    // FIXME: (- 5) should evaluate to -5
    let mut args_unwrapped = args.into_iter()
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

fn do_compare(op_str: &str, args: Vec<SExpr>) -> SExpr {
    let result = if let (SExpr::Atom(arg1), SExpr::Atom(arg2)) = (&args[0], &args[1]) {
        match op_str {
            "<"  => arg1 < arg2,
            ">"  => arg1 > arg2,
            "<=" => arg1 <= arg2,
            ">=" => arg1 >= arg2,
            _ => panic!("Expected an ordering operation, got {}", op_str)
        }
    } else {
        panic!("Expected an atom, found something else.");
    };

    SExpr::Atom(Token::Boolean(result))
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn sub(x: f64, y: f64) -> f64 {
    x - y
}

fn mult(x: f64, y: f64) -> f64 {
    x * y
}

fn div(x: f64, y: f64) -> f64 {
    x / y
}
