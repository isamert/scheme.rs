use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use parser::SExpr;
use parser::SExprs;

pub type VarName = String;
pub type EnvValues = HashMap<VarName, SExpr>;
pub type EnvRef = Rc<RefCell<Option<Env>>>;

#[derive(Debug)]
pub struct Env {
    parent: EnvRef,
    values: EnvValues,
}

pub trait EnvRefT {
    fn clone_ref(&self) -> EnvRef;

    fn get(&self, &str) -> Option<SExpr>;
    fn define(&self, String, SExpr);
    fn set(&self, String, SExpr);

    fn is_some(&self) -> bool;
}

impl EnvRefT for EnvRef {
    fn clone_ref(&self) -> EnvRef {
        Rc::clone(self)
    }

    fn is_some(&self) -> bool {
        self.borrow()
            .as_ref()
            .is_some()
    }

    fn get(&self, name: &str) -> Option<SExpr> {
        self.borrow()
            .as_ref()
            .expect("Cannot find environment")
            .get(name)
    }

    fn define(&self, key: String, val: SExpr) {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .define(key, val);
    }

    fn set(&self, key: String, val: SExpr) {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .set(key, val);
    }
}


impl Env {
    /// A null environment.
    /// Used as parent environment of global environment.
    pub fn null() -> EnvRef {
        Rc::new(RefCell::new(None))
    }

    pub fn new(parent: EnvRef) -> Env {
        Env {
            parent: parent,
            values: HashMap::new(),
        }
    }

    pub fn with_values(parent: EnvRef, values: EnvValues) -> Env {
        Env {
            parent: parent,
            values: values
        }
    }

    /// Converts `Env` into a `EnvRef`.
    /// This function moves `Env` into a `RefCell`.
    /// If you need another pointer to newly created EnvRef,
    /// use `EnvRef::clone_ref()` which only copies the pointer,
    /// not the environment itself.
    pub fn to_ref(self) -> EnvRef {
        Rc::new(RefCell::new(Some(self)))
    }

    pub fn get(&self, name: &str) -> Option<SExpr> {
        if self.values.contains_key(name) {
            Some(self.values.get(name).unwrap().clone())
        } else {
            if self.parent.is_some() {
                self.parent.get(name)
            } else {
                None
            }
        }
    }

    pub fn define(&mut self, key: String, val: SExpr) {
        self.values.insert(key, val);
    }

    pub fn set(&mut self, key: String, val: SExpr) {
        if let Some(x) = self.values.get_mut(&key) {
            *x = val;
        } else {
            panic!("Unbound variable: {}", key);
        }
    }

    pub fn pack(&mut self, keys: &Vec<String>, vals: SExprs) {
        for (i, arg) in vals.into_iter().enumerate() {
            self.values.insert(keys[i].clone(), arg);
        }
    }
}
