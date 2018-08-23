use std::fmt;
use std::char;

use serr::{SErr, SResult};

// FIXME: only works for integer
// https://stackoverflow.com/a/50282149/3716130
pub struct Radix {
    x: f64,
    radix: u32,
}

impl Radix {
    pub fn new(x: f64, radix: u32) -> SResult<Self> {
        if radix < 2 || radix > 36 {
            bail!(Generic => "Unsupported radix")
        } else {
            Ok(Self { x, radix })
        }
    }
}

impl fmt::Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = self.x;
        // Good for binary formatting of `u128`s
        let mut result = ['\0'; 128];
        let mut used = 0;
        let negative = x < 0.0;
        if negative {
            x*=-1.0;
        }
        let mut x = x as u32;
        loop {
            let m = x % self.radix;
            x /= self.radix;

            result[used] = char::from_digit(m, self.radix).unwrap();
            used += 1;

            if x == 0 {
                break;
            }
        }

        if negative {
            write!(f, "-")?;
        }

        for c in result[..used].iter().rev() {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}
