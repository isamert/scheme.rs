use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use parser::SExpr;

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

    fn get(&self, &str) -> SExpr;
    fn insert(&self, String, SExpr);
}

impl EnvRefT for EnvRef {
    fn clone_ref(&self) -> EnvRef {
        Rc::clone(self)
    }

    fn get(&self, name: &str) -> SExpr {
        self.borrow()
            .as_ref()
            .expect("Cannot find environment")
            .get(name)
    }

    fn insert(&self, key: String, val: SExpr) {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .insert(key, val);
    }
}


impl Env {
    /// A null environment.
    /// Used as parent environment of global environment.
    pub fn null() -> EnvRef {
        Rc::new(RefCell::new(None))
    }

    /// Converts `Env` into a `EnvRef`.
    /// This function moves `Env` into a `RefCell`.
    /// If you need another pointer to newly created EnvRef,
    /// use `EnvRef::clone_ref()` which only copies the pointer,
    /// not the environment itself.
    pub fn to_ref(self) -> EnvRef {
        Rc::new(RefCell::new(Some(self)))
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

    pub fn get(&self, name: &str) -> SExpr {
        if self.values.contains_key(name) {
            self.values.get(name).unwrap().clone()
        } else {
            self.parent.get(name)
        }
    }

    pub fn insert(&mut self, key: String, val: SExpr) {
        self.values.insert(key, val);
    }

    pub fn pack(&mut self, keys: &Vec<String>, vals: Vec<SExpr>) {
        for (i, arg) in vals.into_iter().enumerate() { 
            self.values.insert(keys[i].clone(), arg); 
        }
    }
}
