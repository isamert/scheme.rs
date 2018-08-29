use env::Env;
use env::EnvRef;
use lexer::Token;
use parser::SExpr;
use parser::SExprs;
use evaluator::Args;
use serr::{SErr, SResult};

type PrimitiveProcedure = fn(Args) -> SResult<SExpr>;

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
    pub body: Box<SExpr>,
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
    pub fn new_compound(params_expr: SExpr, mut body: SExprs, env: &EnvRef) -> SResult<SExpr> {
        let params = match params_expr {
            SExpr::Atom(Token::Symbol(x)) => {
                Param::Single(x)
            },
            SExpr::List(xs) => {
                let names = xs.into_iter()
                    .map(|x| x.into_symbol())
                    .collect::<SResult<_>>()?;

                Param::Fixed(names)
            },
            SExpr::DottedList(xs, y) => {
                let names = xs.into_iter()
                    .map(|x| x.into_symbol())
                    .collect::<SResult<_>>()?;

                // FIXME: what if its an another list or dotted list?
                let rest = y.into_symbol()?;
                Param::Multi(names, rest)
            },
            x => bail!(TypeMismatch => "parameter list", x)
        };


        // Wrap body in begin: (begin body)
        let body_expr = if body.len() == 1 {
            body.into_iter().next().unwrap()
        } else {
            let mut body_vec = vec![ssymbol!("begin")];
            body_vec.append(&mut body);
            SExpr::List(body_vec)
        };


        let proc = SExpr::Procedure(ProcedureData::Compound(CompoundData {
            params,
            body: Box::new(body_expr),
            env: env.clone_ref()
        }));

        Ok(proc)
    }

    /// Creates a primitive function,
    /// a `SExpr::Procedure(ProcedureData::Primitive)`
    pub fn new_primitive(fun: PrimitiveProcedure) -> SExpr {
        SExpr::Procedure(ProcedureData::Primitive(PrimitiveData { fun }))
    }

    pub fn apply(&self, args: Args) -> SResult<SExpr> {
        match self {
            ProcedureData::Primitive(x) => x.apply(args),
            ProcedureData::Compound(x) => x.apply(args),
        }
    }
}

impl CompoundData {
    pub fn build_env(&self, args: Args) -> SResult<EnvRef> {
        let mut inner_env = Env::new(self.env.clone_ref());
        match self.params {
            Param::Single(ref x) => {
                inner_env.define(x.to_string(), SExpr::List(args.eval()?));
            },
            Param::Fixed(ref xs) => {
                if xs.len() != args.len() {
                    bail!(WrongArgCount => xs.len(), args.len())
                }
                inner_env.pack(xs.as_slice(), args.eval()?);
            },
            Param::Multi(ref xs, ref y) => {
                if args.len() < xs.len() {
                    bail!(WrongArgCount => xs.len(), args.len())
                }

                let mut evaled_args = args.eval()?.into_iter();
                for name in xs {
                    inner_env.define(name.clone(), evaled_args.next().unwrap());
                }

                let rest = evaled_args.take_while(|_| true).collect::<SExprs>();
                inner_env.define(y.clone(), SExpr::List(rest));
            }
        };

        Ok(inner_env.into_ref())
    }

    pub fn apply(&self, args: Args) -> SResult<SExpr> {
        let inner_env = self.build_env(args)?;
        self.body.eval(&inner_env)
    }
}


impl PrimitiveData {
    pub fn apply(&self, args: Args) -> SResult<SExpr> {
        (self.fun)(args)
    }
}
