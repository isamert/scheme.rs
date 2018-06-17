use lexer::Token;
use parser::SExpr;
use evaluator::Args;
use procedure::ProcedureData;
use env::EnvRefT;

pub fn define(args: Args) -> SExpr {
    let name_expr = args.get(0)
        .expect("Expected an identifier, found nothing.");

    let name = if let SExpr::Atom(Token::Symbol(name)) = name_expr {
        name 
    } else {
        panic!("Expected an identifier, not an expr.")
    };

    let value = args.get(1)
        .expect("Expected an expression, found nothing.");

    let value_sexpr = value.eval(&args.env);

    args.env.insert(name.to_string(), value_sexpr.clone());

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

    ProcedureData::new(params, body, &args.env)
}


pub fn quote(args: Args) -> SExpr {
    if args.len() != 1 {
        panic!("Wrong number of arguments while using `quote`.");
    }

    args.get(0)
        .unwrap()
        .clone()
}
