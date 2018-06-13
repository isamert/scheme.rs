use std::rc::Rc;

use env::Env;
use env::EnvRef;
use env::EnvRefT;
use parser::SExpr;
use evaluator;

/// A `Procedure` may be either primitive or compound(user-defined).
#[derive(Debug, Clone)]
pub enum ProcedureData {
    Primitive(PrimitiveData),
    Compound(CompoundData)
}

#[derive(Debug, Clone)]
pub struct PrimitiveData {
    fun: (fn(Vec<SExpr>) -> SExpr),
}

#[derive(Debug, Clone)]
pub struct CompoundData {
    params: Vec<String>,
    body: Vec<SExpr>,
    env: EnvRef
}

impl ProcedureData {
    pub fn new(params: Vec<String>, body: Vec<SExpr>, env: EnvRef) -> SExpr {
        SExpr::Procedure(ProcedureData::Compound(CompoundData {
            params: params,
            body: body,
            env: env
        }))
    }

    pub fn new_primitive(fun: (fn(Vec<SExpr>) -> SExpr)) -> SExpr {
        SExpr::Procedure(ProcedureData::Primitive(PrimitiveData {
            fun: fun
        }))
    }

    pub fn run(&self, args: Vec<SExpr>) -> SExpr {
        match self {
            ProcedureData::Compound(x)   => x.run(args),
            ProcedureData::Primitive(x) => x.run(args)
        }
    }
}

impl CompoundData {
    pub fn run(&self, args: Vec<SExpr>) -> SExpr {
        if self.params.len() != args.len() {
            panic!("Argument count is different than expected.");
        }

        let mut inner_env = Env::new(self.env.clone_ref()); 
        inner_env.pack(&self.params, args);


        // FIXME: Definitions in closureses must be at the top level
        // But this does not check it
        let mut last_expr = None;
        let env_ref = inner_env.to_ref();
        for (i, expr) in self.body.iter().enumerate() {
            last_expr = Some(evaluator::eval(expr, env_ref.clone_ref()));
        }

        last_expr.unwrap()
    }
}


impl PrimitiveData {
    pub fn run(&self, args: Vec<SExpr>) -> SExpr {
        (self.fun)(args)
    }
}

