use std::collections::HashMap;
use std::rc::Rc;

use parser::SExpr;
use parser::SExprs;
use serr::{SErr, SResult};
use utils::{new_rc_ref_cell, RcRefCell};

pub type VarName = String;
pub type EnvValues = HashMap<VarName, SExpr>;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvRef(RcRefCell<Option<Env>>);

impl EnvRef {
    /// A null environment.
    /// Used as parent environment of global environment.
    pub fn null() -> EnvRef {
        EnvRef(new_rc_ref_cell(None))
    }

    pub fn new(env: Env) -> EnvRef {
        EnvRef(new_rc_ref_cell(Some(env)))
    }

    pub fn is_some(&self) -> bool {
        self.0.borrow().as_ref().is_some()
    }

    pub fn clone_ref(&self) -> EnvRef {
        EnvRef(Rc::clone(&self.0))
    }

    pub fn get(&self, name: &str) -> SResult<SExpr> {
        self.0.borrow()
            .as_ref()
            .ok_or_else(|| SErr::EnvNotFound)?
            .get(name)
    }

    /// Use this function to get a real reference to what is inside the Environment,
    /// not a copy of it. Useful for Ports particularly.
    /// It's impossible to return a reference to something inside a RefCell.
    /// (Actually it's quite possible trough std::cell::Ref but not in this
    /// particular case) So we need this extra functions.
    pub fn with_ref<F,T>(&self, name: &str, f: F) -> SResult<T>
    where F: FnMut(&SExpr)->SResult<T> {
        self.0.borrow()
            .as_ref()
            .ok_or_else(|| SErr::EnvNotFound)?
            .with_ref(name, f)
    }

    pub fn with_mut_ref<F,T>(&self, name: &str, f: F) -> SResult<T>
    where F: FnMut(&mut SExpr)->SResult<T> {
        self.0.borrow_mut()
            .as_mut()
            .ok_or_else(|| SErr::EnvNotFound)?
            .with_mut_ref(name, f)
    }

    pub fn define(&self, key: String, val: SExpr) {
        self.0.borrow_mut()
            .as_mut()
            .expect("Can't find environment")
            .define(key, val);
    }

    pub fn set(&self, key: String, val: SExpr) -> SResult<SExpr> {
        self.0.borrow_mut()
            .as_mut()
            .ok_or_else(|| SErr::EnvNotFound)?
            .set(key, val)
    }

    pub fn remove(&self, key: &str) -> SResult<SExpr> {
        self.0.borrow_mut()
            .as_mut()
            .ok_or_else(|| SErr::EnvNotFound)?
            .remove(key)
    }
}

#[derive(Debug, PartialEq)]
pub struct Env {
    parent: EnvRef,
    values: EnvValues,
}

impl Env {
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
        EnvRef::new(self)
    }

    pub fn get(&self, name: &str) -> SResult<SExpr> {
        if self.values.contains_key(name) {
            Ok(self.values[name].clone())
        } else if self.parent.is_some() {
            self.parent.get(name)
        } else {
            bail!(UnboundVar => name)
        }
    }

    pub fn with_ref<F,T>(&self, name: &str, mut f: F) -> SResult<T>
    where F: FnMut(&SExpr)->SResult<T> {
        if self.values.contains_key(name) {
            let sexpr = &self.values[name];
            f(sexpr)
        } else if self.parent.is_some() {
            self.parent.with_ref(name, f)
        } else {
            bail!(UnboundVar => name)
        }
    }

    pub fn with_mut_ref<F,T>(&mut self, name: &str, mut f: F) -> SResult<T>
    where F: FnMut(&mut SExpr)->SResult<T>{
        if self.values.contains_key(name) {
            let sexpr = self.values.get_mut(name).unwrap();
            f(sexpr)
        } else if self.parent.is_some() {
            self.parent.with_mut_ref(name, f)
        } else {
            bail!(UnboundVar => name)
        }
    }

    pub fn define(&mut self, key: String, val: SExpr) {
        self.values.insert(key, val);
    }

    pub fn set(&mut self, key: String, val: SExpr) -> SResult<SExpr> {
        if self.values.contains_key(&key) {
            self.values.insert(key.clone(), val)
                .ok_or_else(|| SErr::new_unbound_var(&key))
        } else if self.parent.is_some() {
            self.parent.set(key, val)
        } else {
            bail!(UnboundVar => key)
        }
    }

    pub fn remove(&mut self, key: &str) -> SResult<SExpr> {
        if self.values.contains_key(key) {
            self.values.remove(key)
                .ok_or_else(|| SErr::new_unbound_var(key))
        } else if self.parent.is_some() {
            self.parent.remove(key)
        } else {
            bail!(UnboundVar => key)
        }
    }

    pub fn pack(&mut self, keys: &[String], vals: SExprs) {
        for (i, arg) in vals.into_iter().enumerate() {
            self.values.insert(keys[i].clone(), arg);
        }
    }
}
