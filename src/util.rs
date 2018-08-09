use std::vec::IntoIter;
use std::iter::Peekable;
use std::str::FromStr;
use std::num::ParseIntError; 

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

fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Fraction {
    pub n: i64,
    pub d: i64
}

impl Fraction {
    pub fn new(n: i64, d: i64) -> Self {
        if d == 0 {
            panic!("Divide by zero dude!") }

        if d < 0 {
            Self { n: -n, d: -d }.reduce()
        } else {
            Self { n: n, d: d }.reduce()
        }
    }

    pub fn reduce(&self) -> Self {
        let gcd = gcd(self.n.abs(), self.d.abs());
        Self {
            n: self.n / gcd,
            d: self.d / gcd
        }
    }

    pub fn is_int(&self) -> bool {
        self.d == 1
    }
}

impl FromStr for Fraction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split('/');
        let n = splitted.next().unwrap().parse::<i64>()?;
        let d = splitted.next().unwrap().parse::<i64>()?;

        Ok(Fraction::new(n, d))
    }
}

#[macro_export]
macro_rules! environment(
    { $($key:expr => $value:expr),+ } => {
        {
            use env::EnvValues;
            let mut m = EnvValues::new();
            $(m.insert($key.to_string(), $value);)+m
        }
    };
);

