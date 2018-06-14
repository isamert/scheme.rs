use lexer::Token;
use parser::SExpr;
use evaluator;
use evaluator::Args;
use procedure::ProcedureData;
use env::EnvRefT;

pub fn define(args: Args) -> SExpr {
    let env = args.env();

    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.");

    let name = if let SExpr::Atom(Token::Symbol(name)) = name_expr {
        name 
    } else {
        panic!("Expected an identifier, not an expr.")
    };

    let value = args.get(1)
        .expect("Expected an expression, found nothing.");

    let value_sexpr = evaluator::eval(value, env.clone_ref());

    env.insert(name.to_string(), value_sexpr.clone());

    value_sexpr
}

pub fn lambda(args: Args) -> SExpr {
    let params_expr = args.get(0)
        .expect("Expected a list of parameters, found nothing.");

    let params = if let SExpr::List(params) = params_expr {
        params.iter()
            .map(|x| if let SExpr::Atom(Token::Symbol(symbol)) = x {
                symbol.clone()
            } else {
                panic!("Expected a parameter name, found this: {:#?}", x);
            })
            .collect::<Vec<String>>()
    } else {
        panic!("Expected an identifier, not an expr.")
    };

    let body = args.all()[1..].to_vec();

    ProcedureData::new(params, body, args.env())
}

pub fn if_(args: Args) -> SExpr {
    let condition = args.get(0)
        .expect("Expected a boolean or expression, found nothing");
    let on_true = args.get(1)
        .expect("Expected an expression, found nothing.");
    let on_false = args.get(2)
        .expect("Expected an expression, found nothing.");

    let env = args.env();
    if to_bool(evaluator::eval(condition, env.clone_ref())) {
        evaluator::eval(on_true, env.clone_ref())
    } else {
        evaluator::eval(on_false, env.clone_ref())
    }
}

pub fn quote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    args.get(0)
        .unwrap()
        .clone()
}




// TODO: wrap in a module in this file maybe?
fn to_bool(x: SExpr) -> bool {
    // Anything other than #f is treated as true.
    match x {
        SExpr::Atom(Token::Boolean(x)) => x,
        _ => true
    }
}
