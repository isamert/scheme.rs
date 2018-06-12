use std::rc::Rc;

use env::Env;
use env::EnvRef;
use parser::SExpr;
use evaluator;

#[derive(Debug, Clone)]
pub enum ClosureData {
    Primitive(PrimitiveData),
    Defined(DefinedData)
}

#[derive(Debug, Clone)]
pub struct PrimitiveData {
    fun: (fn(Vec<SExpr>) -> SExpr)
}

#[derive(Debug, Clone)]
pub struct DefinedData {
    params: Vec<String>,
    body: Vec<SExpr>,
    env: EnvRef
}

pub trait Runnable {
    fn run(&self, args: Vec<SExpr>) -> SExpr;
}


impl ClosureData {
    pub fn new(params: Vec<String>, body: Vec<SExpr>, env: EnvRef) -> ClosureData {
        ClosureData::Defined(DefinedData {
            params: params,
            body: body,
            env: env
        })
    }

    pub fn new_primitive(fun: (fn(Vec<SExpr>) -> SExpr)) -> ClosureData {
        ClosureData::Primitive(PrimitiveData {
            fun: fun
        })
    }

}

impl Runnable for ClosureData {
    fn run(&self, args: Vec<SExpr>) -> SExpr {
        match self {
            ClosureData::Defined(x)   => x.run(args),
            ClosureData::Primitive(x) => x.run(args)
        }
    }
}

impl Runnable for DefinedData {
    fn run(&self, args: Vec<SExpr>) -> SExpr {
        if self.params.len() != args.len() {
            panic!("Argument count is different than expected.");
        }

        let mut inner_env = Env::new(Rc::clone(&self.env)); 
        inner_env.pack(&self.params, args);


        // FIXME: Definitions in closureses must be at the top level
        // But this does not check it
        let mut last_expr = None;
        let env_ref = inner_env.to_ref();
        for (i, expr) in self.body.iter().enumerate() {
            last_expr = Some(evaluator::eval(expr, Rc::clone(&env_ref)));
        }

        last_expr.unwrap()
    }
}


impl Runnable for PrimitiveData {
    fn run(&self, args: Vec<SExpr>) -> SExpr {
        (self.fun)(args)
    }
}

