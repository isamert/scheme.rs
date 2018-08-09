use parser::SExpr;
use evaluator::Args;

pub fn eq(args: Args) -> SExpr {
    eqv(args)
}

pub fn eqv(args: Args) -> SExpr {
    equality(args, |args| {
        let evaled = args.eval();
        match (evaled.get(0).unwrap(), evaled.get(1).unwrap()) {
            (SExpr::Atom(x), SExpr::Atom(y)) => x == y,
            (SExpr::List(x), SExpr::List(y)) => x.is_empty() && y.is_empty(),
            (SExpr::Vector(x), SExpr::Vector(y)) => x.is_empty() && y.is_empty(),
            (_,_) => false
        }
    })
}

pub fn equal(args: Args) -> SExpr {
    equality(args, |args| {
        let evaled = args.eval();
        let obj1 = evaled.get(0).unwrap();
        let obj2 = evaled.get(1).unwrap();

        obj1 == obj2
    })
}

fn equality<F>(args: Args, mut non_atom: F) -> SExpr
where F: (FnMut(&Args) -> bool) {
    if args.len() < 2 {
        return SExpr::boolean(true);
    }

    let obj1 = args.get(0).unwrap();
    let obj2 = args.get(1).unwrap();

    let result = match (obj1, obj2) {
        (SExpr::Atom(x), SExpr::Atom(y)) => x == y,
        _ => {
            non_atom(&args)
        }
    };

    SExpr::boolean(result)
}
