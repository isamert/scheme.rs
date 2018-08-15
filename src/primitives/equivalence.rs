use parser::SExpr;
use evaluator::Args;

pub fn eq(args: Args) -> SExpr {
    eqv(args)
}

pub fn eqv(args: Args) -> SExpr {
    equality(args, |args| {
        let evaled = args.eval();
        match (&evaled[0], &evaled[1]) {
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
        let obj1 = &evaled[0];
        let obj2 = &evaled[1];

        obj1 == obj2
    })
}

fn equality<F>(args: Args, mut non_atom: F) -> SExpr
where F: (FnMut(&Args) -> bool) {
    if args.len() < 2 {
        return SExpr::boolean(true);
    }

    let result = match (&args[0], &args[1]) {
        (SExpr::Atom(x), SExpr::Atom(y)) => x == y,
        _ => {
            non_atom(&args)
        }
    };

    SExpr::boolean(result)
}
