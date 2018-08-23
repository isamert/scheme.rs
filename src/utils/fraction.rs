use std::num::ParseIntError;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use std::f64;

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

impl From<f64> for Fraction {
    // https://rosettacode.org/wiki/Convert_decimal_number_to_rational#Rust
    fn from(mut n: f64) -> Fraction {
        let flag_neg  = n < 0.0;
        if flag_neg { n = n*(-1.0) }
        if n < f64::MIN_POSITIVE {
            return Fraction::new(0, 1)
        }
        if (n - n.round()).abs() < f64::EPSILON {
            return Fraction::new(n.round() as i64, 1)
        }
        let mut a : isize = 0;
        let mut b : isize = 1;
        let mut c : isize = n.ceil() as isize;
        let mut d : isize = 1;
        let aux1 = isize::max_value()/2;
        while c < aux1  && d < aux1 {
            let aux2 : f64 = (a as f64 + c as f64)/(b as f64 + d as f64);
            if (n - aux2).abs() < f64::EPSILON { break }
            if n > aux2 {
                a = a + c;
                b = b + d;
            } else {
                c = a + c;
                d = b + d;
            }
        }

        if flag_neg {
            Fraction::new(-(a+c) as i64, (b+d) as i64)
        } else {
            Fraction::new((a+c) as i64, (b+d) as i64)
        }
    }
}
