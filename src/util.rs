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

#[macro_export]
macro_rules! environment(
    { $($key:expr => $value:expr),+ } => {
        {
            use env::EnvValues;
            let mut m = EnvValues::new();
            $(m.insert($key, $value);)+m
        }
    };
);
