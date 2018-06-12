use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use parser::SExpr;

pub type VarName = String;
pub type EnvRef = Rc<RefCell<Option<Env>>>;

#[derive(Debug)]
pub struct Env {
    parent: EnvRef,
    values: HashMap<VarName, SExpr>,
}

// TODO: Construnct Env with values and use Vec::with_capacity
impl Env {
    pub fn new(parent: EnvRef) -> Env {
        Env {
            parent: parent,
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> SExpr {
        if self.values.contains_key(name) {
            self.values.get(name).unwrap().clone()
        } else {
            self.parent.borrow()
                .as_ref()
                .unwrap()
                .get(name)
        }
    }

    pub fn insert(&mut self, key: String, val: SExpr) {
        self.values.insert(key, val);
    }
}
