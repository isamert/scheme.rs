#[macro_use]
pub mod macros;
pub mod fraction;
pub mod funcs;
pub mod chars;

use std::vec::IntoIter;
use std::iter::Peekable;

pub trait GentleIterator<I: Iterator> {
    fn take_until<F>(&mut self, predicate: F) -> IntoIter<I::Item>
        where F: Fn(&I::Item) -> bool;
}

impl<I: Iterator> GentleIterator<I> for Peekable<I> {
    fn take_until<F>(&mut self, predicate: F) -> IntoIter<I::Item>
        where F: Fn(&I::Item) -> bool {

        let mut v: Vec<I::Item> = vec![];
        while self.peek().map_or(false, &predicate) {
                v.push(self.next().unwrap());
        }

        v.into_iter()
    }
}


pub trait AndOr<U> {
    fn and_or(self, optb: Option<U>) -> Option<U>;
}

impl<U> AndOr<U> for Option<U> {
    /// Returns `optb` if `optb` and option is `Some`.
    /// Returns option if `optb` is `None` and option is `Some`.
    /// Otherwise returns None.
    /// Basically tries to return `Some` at deepest level.
    fn and_or(self, optb: Option<U>) -> Option<U> {
        match self {
            a@Some(_) => match optb {
                b@Some(_) => b,
                None => a
            },
            None => None,
        }
    }
}

