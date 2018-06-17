use env::Env;
use env::EnvRef;
use env::EnvRefT;
use parser::SExpr;
use evaluator::Args;

type PrimitiveProcedure = fn(Args) -> SExpr;

/// A `Procedure` may be either primitive or compound(user-defined).
#[derive(Debug, Clone)]
pub enum ProcedureData {
    Primitive(PrimitiveData),
    Compound(CompoundData)
}

#[derive(Debug, Clone)]
pub struct PrimitiveData {
    fun: PrimitiveProcedure,
}

#[derive(Debug, Clone)]
pub struct CompoundData {
    params: Vec<String>,
    body: Vec<SExpr>,
    env: EnvRef
}

impl ProcedureData {
    /// Creates user defined procedure,
    /// a `SExpr::Procedure(ProcedureData::Compound)`.
    pub fn new(params: Vec<String>, body: Vec<SExpr>, env: &EnvRef) -> SExpr {
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
        if self.params.len() != args.len() {
            panic!("Argument count is different than expected.");
        }

        let mut inner_env = Env::new(self.env.clone_ref()); 
        inner_env.pack(&self.params, args.eval());


        // FIXME: Definitions in closureses must be at the top level
        // But this does not check it
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

