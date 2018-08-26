use std::iter::Peekable;
use std::ops::Not;
use std::cmp::Ordering;

use utils::fraction::Fraction;
use lexer::Token;
use procedure::ProcedureData;
use evaluator;
use env::{EnvRef, VarName};
use port::PortData;
use expander::expand;
use serr::{SErr, SResult};

pub type SExprs = Vec<SExpr>;

#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Atom(Token),
    List(SExprs),
    DottedList(Vec<SExpr>, Box<SExpr>),
    Procedure(ProcedureData),
    Port(PortData),
    Lazy(Box<SExpr>),
    Unspecified
}

impl PartialOrd for SExpr {
    fn partial_cmp(&self, other: &SExpr) -> Option<Ordering> {
        use self::SExpr::*;
        match (self, other) {
            (Atom(t1), Atom(t2)) => t1.partial_cmp(t2),
            (_a, _b) => None
        }
    }
}

impl Not for SExpr {
    type Output = SResult<SExpr>;
    fn not(self) -> SResult<SExpr> {
        match self {
            SExpr::Atom(Token::Boolean(x)) => Ok(sbool!(!x)),
            _ => bail!(TypeMismatch => "boolean", self)
        }
    }
}

/// One can call .into() on everything and it yields itself, or it yields
/// the target type if it implements the required From/Into trait.
/// .into is particularly useful in macros. With that implementation in hand,
/// I can send a &SExpr to a macro which calls .into() on everything and get
/// SExpr if it is needed. String/str also implements .into() in same fashion.
impl<'a> From<&'a SExpr> for SExpr {
    fn from(e: &SExpr) -> Self {
        e.clone()
    }
}

impl<'a> From<&'a mut SExpr> for SExpr {
    fn from(e: & mut SExpr) -> Self {
        e.clone()
    }
}

impl From<i64> for SExpr {
    fn from(i: i64) -> Self {
        SExpr::Atom(Token::Integer(i))
    }
}

impl From<f64> for SExpr {
    fn from(i: f64) -> Self {
        SExpr::Atom(Token::Float(i))
    }
}

impl From<Fraction> for SExpr {
    fn from(i: Fraction) -> Self {
        SExpr::Atom(Token::Fraction(i))
    }
}

impl From<char> for SExpr {
    fn from(c: char) -> Self {
        SExpr::Atom(Token::Chr(c))
    }
}

impl From<bool> for SExpr {
    fn from(b: bool) -> Self {
        SExpr::Atom(Token::Boolean(b))
    }
}

impl From<usize> for SExpr {
    fn from(u: usize) -> Self {
        SExpr::Atom(Token::Integer(u as i64))
    }
}

impl From<String> for SExpr {
    fn from(s: String) -> Self {
        SExpr::Atom(Token::Str(s))
    }
}

#[allow(dead_code)]
impl SExpr {
    pub fn dottedlist(x: SExprs, y: SExpr) -> SExpr {
        SExpr::DottedList(x, Box::new(y))
    }

    pub fn to_bool(&self) -> bool {
        // Anything other than #f is treated as true.
        match self {
            SExpr::Atom(Token::Boolean(x)) => *x,
            _ => true
        }
    }

    // Checks
    pub fn is_symbol(&self, symbol: &str) -> bool {
        match self {
            SExpr::Atom(Token::Symbol(x)) => x == symbol,
            _ => false
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            SExpr::Atom(Token::Str(_)) => true,
            _ => false
        }
    }

    pub fn is_chr(&self) -> bool {
        match self {
            SExpr::Atom(Token::Chr(_)) => true,
            _ => false
        }
    }

    pub fn is_ellipsis(&self) -> bool {
        match self {
            SExpr::Atom(Token::Ellipsis) => true,
            _ => false
        }
    }

    pub fn is_lazy(&self) -> bool {
        match self {
            SExpr::Lazy(_) => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            SExpr::Atom(Token::Integer(_)) => true,
            _ => false
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            SExpr::List(xs) if !xs.is_empty() => true,
            SExpr::DottedList(_, _) => true,
            _ => false
        }
    }

    pub fn is_proper_list(&self) -> bool {
        match self {
            SExpr::List(_) => true,
            SExpr::DottedList(_, y) => match &**y {
                SExpr::List(xs) if xs.is_empty() => true,
                _ => false
            }
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            SExpr::Atom(Token::Boolean(_)) => true,
            _ => false
        }
    }

    pub fn is_unspecified(&self) -> bool {
        match self {
            SExpr::Unspecified => true,
            _ => false
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            SExpr::Atom(Token::Integer(_)) | SExpr::Atom(Token::Fraction(_))
                | SExpr::Atom(Token::Float(_)) => true,
            _ => false
        }
    }

    // Borrows
    pub fn as_port(&self) -> SResult<&PortData> {
        match self {
            SExpr::Port(ref pd) => Ok(pd),
            x => bail!(TypeMismatch => "port", x)
        }
    }

    pub fn as_port_mut(&mut self) -> SResult<&mut PortData> {
        match self {
            SExpr::Port(ref mut pd) => Ok(pd),
            x => bail!(TypeMismatch => "port", x)
        }
    }

    pub fn as_symbol(&self) -> SResult<String> {
        match self {
            SExpr::Atom(Token::Symbol(x)) => Ok(x.to_string()),
            x => bail!(TypeMismatch => "string", x)
        }
    }

    pub fn as_int(&self) -> SResult<i64> {
        match self {
            SExpr::Atom(Token::Integer(x)) => Ok(*x),
            x => bail!(TypeMismatch => "integer", x)
        }
    }

    pub fn as_proc(&self) -> SResult<&ProcedureData> {
        match self {
            SExpr::Procedure(x) => Ok(x),
            x => bail!(TypeMismatch => "procedure", x)
        }
    }

    pub fn as_mut_string(&mut self) -> SResult<&mut String> {
        match self {
            SExpr::Atom(Token::Str(x)) => Ok(x),
            x => bail!(TypeMismatch => "procedure", x)
        }
    }


    // Transforms
    pub fn into_symbol(self) -> SResult<String> {
        match self {
            SExpr::Atom(Token::Symbol(x)) => Ok(x),
            x => bail!(TypeMismatch => "symbol", x)
        }
    }

    pub fn into_list(self) -> SResult<SExprs> {
        match self {
            SExpr::List(xs) => Ok(xs),
            x => bail!(TypeMismatch => "list", x)
        }
    }

    pub fn into_str(self) -> SResult<String> {
        match self {
            SExpr::Atom(Token::Str(x)) => Ok(x),
            x => bail!(TypeMismatch => "string", x)
        }
    }

    pub fn into_chr(self) -> SResult<char> {
        match self {
            SExpr::Atom(Token::Chr(x)) => Ok(x),
            x => bail!(TypeMismatch => "char", x)
        }
    }

    pub fn into_int(self) -> SResult<i64> {
        match self {
            SExpr::Atom(Token::Integer(x)) => Ok(x),
            x => bail!(TypeMismatch => "int", x)
        }
    }

    pub fn into_float(self) -> SResult<f64> {
        match self {
            SExpr::Atom(Token::Float(x)) => Ok(x),
            SExpr::Atom(Token::Integer(x)) => Ok(x as f64),
            SExpr::Atom(Token::Fraction(x)) => Ok(x.into()),
            x => bail!(TypeMismatch => "float", x)
        }
    }
    // Transform operations
    pub fn list_own_one_rest(self) -> SResult<(SExpr, SExprs)> {
        match self {
            SExpr::List(xs) => {
                let mut iter = xs.into_iter();
                let head = iter.next()
                    .ok_or_else(|| SErr::FoundNothing)?;
                let tail = iter.collect();

                Ok((head, tail))
            }
            x => bail!(TypeMismatch => "list", x)
        }
    }

    pub fn eval(&self, env: &EnvRef) -> SResult<SExpr> {
        evaluator::eval(self, env)
    }

    pub fn eval_ref<F,T>(&self, env: &EnvRef, f: F) -> SResult<T>
    where F: FnMut(&SExpr)->SResult<T> {
        evaluator::eval_ref(self, env, f)
    }

    pub fn eval_mut_ref<F,T>(&self, env: &EnvRef, f: F) -> SResult<T>
    where F: FnMut(&mut SExpr)->SResult<T> {
        evaluator::eval_mut_ref(self, env, f)
    }
}

pub fn parse(tokens: Vec<Token>) -> SResult<SExprs> {
    let mut iter = tokens.into_iter().peekable();
    let mut exprs: SExprs = vec![];

    while let Some(_) = iter.peek() {
        exprs.push(expand(parse_single(&mut iter)?)?);
    }

    Ok(exprs)
}

pub fn parse_single<I>(iter: &mut Peekable<I>) -> SResult<SExpr>
where I: Iterator<Item=Token> {
    match iter.peek() {
        Some(&Token::RParen) => bail!(UnexpectedToken => Token::RParen),
        Some(&Token::LParen) => {
            iter.next(); // Consume LParen

            // Check if empty list
            if iter.peek() == Some(&Token::RParen) {
                iter.next(); // Consume RParen
                return Ok(slist![]);
            }

            let mut head: SExprs = vec![];
            while iter.peek() != Some(&Token::RParen) &&
                    iter.peek() != Some(&Token::Dot) {
                head.push(parse_single(iter)?);
            }

            match iter.next() {
                Some(Token::Dot) => {
                    let tail = parse_single(iter)?;
                    if iter.peek() != Some(&Token::RParen) {
                        let unexpected = iter.peek().unwrap().clone();
                        bail!(NotExpectedToken => unexpected, Token::RParen)
                    } else {
                        iter.next(); // Consume RParen

                        // If the tail is a proper list, then the result should
                        // also be a proper list.
                        if let SExpr::List(mut xs) = tail {
                            head.append(&mut xs);
                            Ok(SExpr::List(head))
                        } else {
                            Ok(SExpr::DottedList(head, Box::new(tail)))
                        }
                    }
                },
                Some(Token::RParen) => {
                    Ok(SExpr::List(head))
                },
                x => bail!(UnexpectedToken => x.unwrap()),
            }
        },
        Some(&Token::Quote) => {
            iter.next();
            Ok(quote!(parse_single(iter)?))
        },
        Some(&Token::UnQuote) => {
            iter.next();
            Ok(unquote!(parse_single(iter)?))
        },
        Some(&Token::QuasiQuote) => {
            iter.next();
            Ok(quasiquote!(parse_single(iter)?))
        },
        Some(&Token::UnQuoteSplicing) => {
            iter.next();
            Ok(unquote_splicing!(parse_single(iter)?))
        },
        Some(_) => {
            let y = iter.next().unwrap();
            Ok(SExpr::Atom(y))
        },
        None => serr!(FoundNothing)
    }
}
