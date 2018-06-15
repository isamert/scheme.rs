use parser::SExpr;
use evaluator;
use evaluator::Args;

pub fn list(args: Args) -> SExpr {
    let list : Vec<SExpr> = args.all()
        .iter()
        .map(|x| evaluator::eval(x, args.env()))
        .collect();

    SExpr::List(list)
}

// TODO: generalize following functions, using SliceIndex?
pub fn car(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong argument count to car.");
    }

    let list = &args.eval()[0];
    match list {
        SExpr::List(x) => x[0].clone(),
        _              => panic!("Wrong type of argument to car.")
    }
}

pub fn cdr(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong argument count to cdr.");
    }

    let list = &args.eval()[0];
    match list {
        SExpr::List(x) => {
            let result = x.get(1..)
                .expect("The list is not big enough.")
                .to_vec();
                
            SExpr::List(result)
        },
        _ => panic!("Wrong type of argument to cdr.")
    }
}
