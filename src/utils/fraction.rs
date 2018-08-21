use std::num::ParseIntError;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

use utils::funcs::gcd;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Fraction {
    pub n: i64,
    pub d: i64
}

impl Fraction {
    pub fn new(n: i64, d: i64) -> Self {
        if d == 0 {
            panic!("Divide by zero dude!")
        }

        if d < 0 {
            Self { n: -n, d: -d }.reduce()
        } else {
            Self { n, d }.reduce()
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

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Fraction::new(self.n * rhs.d + rhs.n * self.d, self.d * rhs.d)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Fraction::new(self.n * rhs.d - rhs.n * self.d, self.d * rhs.d)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Fraction::new(self.n * rhs.n, self.d * rhs.d)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Fraction::new(self.n * rhs.d, self.d * rhs.n)
    }
}

impl From<i64> for Fraction {
    fn from(i: i64) -> Fraction {
        Fraction::new(i, 1)
    }
}

impl From<Fraction> for f64 {
    fn from(f: Fraction) -> f64 {
        f.n as f64 / f.d as f64
    }
}
