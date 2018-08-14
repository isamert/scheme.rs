use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use parser::SExpr;
use parser::SExprs;

pub type VarName = String;
pub type EnvValues = HashMap<VarName, SExpr>;
pub type EnvRef = Rc<RefCell<Option<Env>>>;

#[derive(Debug, PartialEq)]
pub struct Env {
    parent: EnvRef,
    values: EnvValues,
}

pub trait EnvRefT {
    fn clone_ref(&self) -> EnvRef;

    fn get(&self, &str) -> Option<SExpr>;
    fn with_ref<F,T>(&self, name: &str, j: F) -> T where F: FnMut(Option<&SExpr>)->T;
    fn with_mut_ref<F,T>(&self, name: &str, f: F) -> T where F: FnMut(Option<&mut SExpr>)->T;
    fn define(&self, String, SExpr);
    fn set(&self, String, SExpr) -> Option<SExpr>;
    fn remove(&self, &str) -> Option<SExpr>;

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

    /// Use this function to get a real reference to what is inside the Environment,
    /// not a copy of it. Useful for Ports particularly.
    /// It's impossible to return a reference to something inside a RefCell.
    /// (Actually it's quite possible trough std::cell::Ref but not in this
    /// particular case) So we need this extra functions.
    fn with_ref<F,T>(&self, name: &str, f: F) -> T
    where F: FnMut(Option<&SExpr>)->T {
        self.borrow()
            .as_ref()
            .expect("Cannot find environment")
            .with_ref(name, f)
    }

    fn with_mut_ref<F,T>(&self, name: &str, f: F) -> T
    where F: FnMut(Option<&mut SExpr>)->T {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .with_mut_ref(name, f)
    }

    fn define(&self, key: String, val: SExpr) {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .define(key, val);
    }

    fn set(&self, key: String, val: SExpr) -> Option<SExpr> {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .set(key, val)
    }

    fn remove(&self, key: &str) -> Option<SExpr> {
        self.borrow_mut()
            .as_mut()
            .expect("Cannot find environment")
            .remove(key)
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
            parent,
            values: HashMap::new(),
        }
    }

    pub fn with_values(parent: EnvRef, values: EnvValues) -> Env {
        Env { parent, values }
    }

    /// Converts `Env` into a `EnvRef`.
    /// This function moves `Env` into a `RefCell`.
    /// If you need another pointer to newly created EnvRef,
    /// use `EnvRef::clone_ref()` which only copies the pointer,
    /// not the environment itself.
    pub fn into_ref(self) -> EnvRef {
        Rc::new(RefCell::new(Some(self)))
    }

    pub fn get(&self, name: &str) -> Option<SExpr> {
        if self.values.contains_key(name) {
            Some(self.values[name].clone())
        } else if self.parent.is_some() {
            self.parent.get(name)
        } else {
            None
        }
    }

    pub fn with_ref<F,T>(&self, name: &str, mut f: F) -> T
    where F: FnMut(Option<&SExpr>)->T {
        if self.values.contains_key(name) {
            let sexpr = &self.values[name];
            f(Some(sexpr))
        } else if self.parent.is_some() {
            self.parent
                .borrow()
                .as_ref()
                .expect("zaxd")
                .with_ref(name, f)
        } else {
            f(None)
        }
    }

    pub fn with_mut_ref<F,T>(&mut self, name: &str, mut f: F) -> T
    where F: FnMut(Option<&mut SExpr>)->T{
        if self.values.contains_key(name) {
            let sexpr = self.values.get_mut(name).unwrap();
            f(Some(sexpr))
        } else if self.parent.is_some() {
            self.parent
                .borrow_mut()
                .as_mut()
                .expect("zaxd")
                .with_mut_ref(name, f)
        } else {
            f(None)
        }
    }

    pub fn define(&mut self, key: String, val: SExpr) {
        self.values.insert(key, val);
    }

    pub fn set(&mut self, key: String, val: SExpr) -> Option<SExpr> {
        if self.values.contains_key(&key) {
            self.values.insert(key, val)
        } else if self.parent.is_some() {
            self.parent.set(key, val)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<SExpr> {
        if self.values.contains_key(key) {
            self.values.remove(key)
        } else if self.parent.is_some() {
            self.parent.remove(key)
        } else {
            None
        }
    }

    pub fn pack(&mut self, keys: &[String], vals: SExprs) {
        for (i, arg) in vals.into_iter().enumerate() {
            self.values.insert(keys[i].clone(), arg);
        }
    }
}
