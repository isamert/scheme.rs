use lexer::Token;
use parser::SExpr;
use evaluator::Args;

pub fn lt(args: Args) -> SExpr {
    compare("<", args)
}

pub fn gt(args: Args) -> SExpr {
    compare(">", args)
}

pub fn lte(args: Args) -> SExpr {
    compare("<=", args)
}

pub fn gte(args: Args) -> SExpr {
    compare(">=", args)
}

pub fn eq(args: Args) -> SExpr {
    compare("=", args)
}

// TODO: wrap in a helper module?
fn compare(op_str: &str, args: Args) -> SExpr {
    let evaled_args = args.eval();
    let result = if let (SExpr::Atom(arg1), SExpr::Atom(arg2)) = (&evaled_args[0], &evaled_args[1]) {
        match op_str {
            "<"  => arg1 < arg2,
            ">"  => arg1 > arg2,
            "<=" => arg1 <= arg2,
            ">=" => arg1 >= arg2,
            "="  => arg1 == arg2,
            _ => panic!("Expected an ordering operation, got {}", op_str)
        }
    } else {
        panic!("Expected an atom, found something else.");
    };

    SExpr::Atom(Token::Boolean(result))
}
