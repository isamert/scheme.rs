use parser::SExpr;
use evaluator::Args;

pub fn list(args: Args) -> SExpr {
    SExpr::List(args.eval())
}

pub fn cons(args: Args) -> SExpr {
    let (x, xs, _rest) = args.evaled()
        .own_two()
        .expect("Expected an obj and a list as arguments, found something else.");

    match xs {
        SExpr::List(mut xs) => {
            xs.insert(0, x);
            SExpr::List(xs)
        },
        SExpr::DottedList(mut xs, y) => {
            xs.insert(0, x);
            SExpr::DottedList(xs, y)
        },
        y => SExpr::DottedList(vec![x], Box::new(y))
    }
}

pub fn car(args: Args) -> SExpr {
    let (xs, _rest) = args.evaled()
        .own_one()
        .expect("Expected a list as argument, found something else.");

    match xs {
        SExpr::List(ys) | SExpr::DottedList(ys, _) => ys.into_iter().next().unwrap(),
        x => panic!("Expected a list as argument, got this: {}", x)
    }
}

pub fn cdr(args: Args) -> SExpr {
    let (xs, _rest) = args.evaled()
        .own_one()
        .expect("Expected a list as argument, found something else.");

    match xs {
        SExpr::List(ys) => SExpr::List(ys.into_iter().skip(1).collect()),
        SExpr::DottedList(ys, y) => {
            if ys.len() == 1 {
                *y
            } else {
                SExpr::DottedList(ys.into_iter().skip(1).collect(), y)
            }
        },
        x => panic!("Expected a list as argument, got this: {}", x)
    }
}
