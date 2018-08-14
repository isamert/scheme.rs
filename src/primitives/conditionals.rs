use parser::SExpr;
use parser::SExprs;
use evaluator::Args;
use evaluator::ToArgs;


pub fn if_(args: Args) -> SExpr {
    let test = args.get(0)
        .expect("Expected a boolean or expression, found nothing");
    let consequent = args.get(1)
        .expect("Expected an expression, found nothing.");
    let alterne = args.get(2)
        .unwrap_or(&SExpr::Unspecified);

    let env = &args.env;
    if test.eval(&env).to_bool() {
        consequent.eval(&env)
    } else {
        alterne.eval(&env)
    }
}

pub fn cond(args: Args) -> SExpr {
    let clauses = args.all()
        .iter()
        .map(|x| {
            if let SExpr::List(clause) = x {
                let mut current = 0;
                let test = clause.get(current)
                    .expect("Expected a <test>.");

                current += 1;
                if clause.len() == 3 {
                    // Consume `=>`
                    // FIXME: check if `clause.get(current)` is => otherwise panic!
                    current += 1;
                }

                let expr = clause.get(current)
                    .expect("Expected an expression.");

                (test, expr)
            } else {
                panic!("Bad case form.")
            }
        });

    let mut else_clause: Option<SExpr> = None;
    for (test, expr) in clauses {
        if test.is_symbol("else") {
            else_clause = Some(expr.clone());
        } else if test.eval(&args.env).to_bool() {
            return expr.eval(&args.env)
        }
    }

    if else_clause.is_some() {
        else_clause.unwrap()
            .eval(&args.env)
    } else {
        SExpr::Unspecified
    }
}

pub fn case(args: Args) -> SExpr {
    let test = args.get(0).unwrap();
    let args_vec: SExprs = args.all()
        .iter()
        .skip(1)
        .map(|clause| {
            if let SExpr::List(xs) = clause {
                let test = SExpr::List(vec![SExpr::symbol("eqv?"), xs[0].clone(), test.clone()]);
                SExpr::List(vec![test, xs[1].clone()])
            } else {
                panic!("Clause is not in desired form.");
            }
        })
        .collect();

    cond(args_vec.to_args(&args.env))
}

pub fn or(args: Args) -> SExpr {
    let env = args.env();
    let result = args
        .into_all()
        .into_iter()
        .any(|x| x.eval(&env).to_bool());

    SExpr::boolean(result)
}

pub fn and(args: Args) -> SExpr {
    let env = args.env();
    let result = args
        .into_all()
        .into_iter()
        .all(|x| x.eval(&env).to_bool());

    SExpr::boolean(result)
}
