use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;
use std::collections::HashMap;

use parser::SExpr;

pub type VarName = String;
pub type EnvValues = HashMap<VarName, SExpr>;
pub type EnvRef = Option<Rc<RefCell<Env>>>;

pub trait EnvRefT {
    fn clone_ref(&self) -> EnvRef;

    fn borrow(&self) -> Ref<Env>;
    fn borrow_mut(&self) -> RefMut<Env>;
}

impl EnvRefT for EnvRef {
    fn clone_ref(&self) -> EnvRef {
        match self {
            None => None,
            Some(ref x) => Some(Rc::clone(x))
        }
    }

    fn borrow(&self) -> Ref<Env> {
        self.as_ref() // Get a reference to Rc inside Option
            .unwrap() // Unwrap it
            .borrow() // Borrow RefCell
    }

    fn borrow_mut(&self) -> RefMut<Env> {
        self.as_ref() // No need for `as_mut` call because thats what RefMut does
            .unwrap()
            .borrow_mut()
    }
}

#[derive(Debug)]
pub struct Env {
    parent: EnvRef,
    values: EnvValues,
}

impl Env {
    /// A null environment.
    /// Used as parent environment of global environment.
    pub fn null() -> EnvRef {
        None
    }

    /// Converts `Env` into a `EnvRef`.
    /// This function moves `Env` into a `RefCell`.
    /// If you need another pointer to newly created EnvRef,
    /// use `EnvRef::clone_ref()` which only copies the pointer,
    /// not the environment itself.
    pub fn to_ref(self) -> EnvRef {
        Some(Rc::new(RefCell::new(self)))
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
            self.parent.as_ref()
                .unwrap()
                .borrow()
                .get(name)
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
