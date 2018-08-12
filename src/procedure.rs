use env::Env;
use env::EnvRef;
use env::EnvRefT;
use parser::SExpr;
use parser::SExprs;
use evaluator::Args;

type PrimitiveProcedure = fn(Args) -> SExpr;

/// A `Procedure` may be either primitive or compound(user-defined).
#[derive(Debug, Clone, PartialEq)]
pub enum ProcedureData {
    Primitive(PrimitiveData),
    Compound(CompoundData)
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveData {
    fun: PrimitiveProcedure,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundData {
    params: Param,
    body: SExprs,
    env: EnvRef
}

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Single(String),
    Fixed(Vec<String>),
    Multi(Vec<String>, String),
}

impl ProcedureData {
    /// Creates user defined procedure,
    /// a `SExpr::Procedure(ProcedureData::Compound)`.
    pub fn new(params_expr: SExpr, body: SExprs, env: &EnvRef) -> SExpr {
        let params = match params_expr {
            SExpr::List(xs) => {
                if xs.len() == 1 {
                    let name = xs.into_iter()
                        .next()
                        .unwrap()
                        .into_symbol()
                        .expect("Expected a symbol found something else.");
                    Param::Single(name)
                } else {
                    let names = xs.into_iter()
                        .map(|x| x.into_symbol()
                                    .expect("Expected a symbol found something else."))
                        .collect();
                    Param::Fixed(names)
                }
            },
            SExpr::DottedList(xs, y) => {
                let names = xs.into_iter()
                    .map(|x| x.into_symbol()
                                .expect("Expected a symbol found something else."))
                    .collect();

                // FIXME: what if its an another list or dotted list?
                let rest = y.into_symbol()
                    .expect("Expected a symbol found something else.");
                Param::Multi(names, rest)
            },
            _ => panic!("Expected a parameter list, found this: {}", params_expr)
        };

        SExpr::Procedure(ProcedureData::Compound(CompoundData {
            params: params,
            body: body,
            env: env.clone_ref()
        }))
    }

    /// Creates a primitive function,
    /// a `SExpr::Procedure(ProcedureData::Primitive)`
    pub fn new_primitive(fun: PrimitiveProcedure) -> SExpr {
        SExpr::Procedure(ProcedureData::Primitive(PrimitiveData {
            fun: fun
        }))
    }

    pub fn apply(&self, args: Args) -> SExpr {
        match self {
            ProcedureData::Compound(x)  => x.apply(args),
            ProcedureData::Primitive(x) => x.apply(args)
        }
    }
}

impl CompoundData {
    pub fn apply(&self, args: Args) -> SExpr {
        let mut inner_env = Env::new(self.env.clone_ref());
        match self.params {
            Param::Single(ref x) => {
                inner_env.define(x.to_string(), SExpr::List(args.eval()));
            },
            Param::Fixed(ref xs) => {
                if xs.len() != args.len() {
                    panic!("");
                }
                inner_env.pack(xs.as_slice(), args.eval());
            },
            Param::Multi(ref xs, ref y) => {
                if args.len() < xs.len() {
                    panic!("");
                }

                let mut evaled_args = args.eval().into_iter();
                for i in 0..xs.len() {
                    inner_env.define(xs[i].clone(), evaled_args.next().unwrap());
                }

                let rest = evaled_args.take_while(|_| true).collect::<SExprs>();
                inner_env.define(y.clone(), SExpr::List(rest));
            }
        }


        let mut last_expr = None;
        let env_ref = inner_env.to_ref();
        for (_i, expr) in self.body.iter().enumerate() {
            last_expr = Some(expr.eval(&env_ref));
        }

        last_expr.unwrap()
    }
}


impl PrimitiveData {
    pub fn apply(&self, args: Args) -> SExpr {
        (self.fun)(args)
    }
}
