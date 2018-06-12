use std::rc::Rc;
use std::cell::RefCell;

use env::Env;
use env::EnvRef;
use parser::SExpr;
use evaluator;

#[derive(Debug, Clone)]
pub struct ClosureData {
    params: Vec<String>,
    // TODO: body must be Vec<SExpr>
    body: Vec<SExpr>,
    env: EnvRef
}


impl ClosureData {
    pub fn new(params: Vec<String>, body: Vec<SExpr>, env: EnvRef) -> ClosureData {
        ClosureData {
            params: params,
            body: body,
            env: env
        }
    }

    pub fn run(&self, args: Vec<SExpr>) -> SExpr {
        if self.params.len() != args.len() {
            panic!("Argument count is different than expected.");
        }

        let mut inner_env = Env::new(Rc::clone(&self.env)); 
        // TODO: inner_env.pack(self.params, args)
        for (i, arg) in args.into_iter().enumerate() { 
            inner_env.insert(self.params[i].clone(), arg); 
        }


        // Definitions in closureses must be at the top level
        // But this does not check it
        // TODO return last expr
        let mut last_expr = None;
        let env_ref = Rc::new(RefCell::new(Some(inner_env)));
        for (i, expr) in self.body.iter().enumerate() {
            last_expr = Some(evaluator::eval(expr, Rc::clone(&env_ref)));
        }

        last_expr.unwrap()
    }
}
