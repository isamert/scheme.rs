use lexer::Token;
use parser::SExpr;
use evaluator::Args;

enum Op {
    EQ, LT, GT, LTE, GTE
}

pub fn lt(args: Args) -> SExpr {
    compare(Op::LT, args)
}

pub fn gt(args: Args) -> SExpr {
    compare(Op::GT, args)
}

pub fn lte(args: Args) -> SExpr {
    compare(Op::LTE, args)
}

pub fn gte(args: Args) -> SExpr {
    compare(Op::GTE, args)
}

pub fn eq(args: Args) -> SExpr {
    compare(Op::EQ, args)
}

// TODO: wrap in a helper module?
fn compare(op: Op, args: Args) -> SExpr {
    let evaled_args = args.eval();
    let result = if let (SExpr::Atom(arg1), SExpr::Atom(arg2)) = (&evaled_args[0], &evaled_args[1]) {
        match op {
            Op::LT  => arg1 < arg2,
            Op::GT  => arg1 > arg2,
            Op::LTE => arg1 <= arg2,
            Op::GTE => arg1 >= arg2,
            Op::EQ  => arg1 == arg2,
        }
    } else {
        panic!("Expected an atom, found something else.");
    };

    SExpr::Atom(Token::Boolean(result))
}
