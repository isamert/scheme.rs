use std::iter::Peekable;
use std::vec::IntoIter;
use std::ops::Deref;
use std::ops::Not;
use std::cmp::Ordering;

use lexer::Token;
use procedure::ProcedureData;
use evaluator;
use env::EnvRef;
use ports::PortData;
use serr::{SErr, SResult};

pub type SExprs = Vec<SExpr>;

// TODO: needs huge refactoring
#[derive(Debug, Clone)]
pub struct Expr {
    sexpr: SExpr,
    data: usize
}

impl Deref for Expr {
    type Target = SExpr;

    fn deref(&self) -> &SExpr {
        &self.sexpr
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Atom(Token),
    List(SExprs),
    DottedList(Vec<SExpr>, Box<SExpr>),
    Vector(Vec<SExpr>),
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
            SExpr::Atom(Token::Boolean(x)) => Ok(SExpr::boolean(!x)),
            _ => bail!(TypeMismatch => "boolean", self)
        }
    }
}

/// One can call .into() into everything and it yields itself, or it yields
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

#[allow(dead_code)]
impl SExpr {
    pub fn symbol(x: &str) -> SExpr {
        SExpr::Atom(Token::Symbol(x.to_string()))
    }

    pub fn integer(x: i64) -> SExpr {
        SExpr::Atom(Token::Integer(x))
    }

    pub fn float(x: f64) -> SExpr {
        SExpr::Atom(Token::Float(x))
    }

    pub fn boolean(x: bool) -> SExpr {
        SExpr::Atom(Token::Boolean(x))
    }

    pub fn chr(x: char) -> SExpr {
        SExpr::Atom(Token::Chr(x))
    }

    pub fn str_(x: &str) -> SExpr {
        SExpr::Atom(Token::Str(x.to_string()))
    }

    pub fn str_owned(x: String) -> SExpr {
        SExpr::Atom(Token::Str(x))
    }

    pub fn quasiquote(mut args: SExprs) -> SExpr {
        args.insert(0, SExpr::symbol("quasiquote"));
        SExpr::List(args)
    }

    pub fn quote(sexpr: SExpr) -> SExpr {
        SExpr::List(vec![SExpr::symbol("quote"), sexpr])
    }

    pub fn unquote(sexpr: SExpr) -> SExpr {
        SExpr::List(vec![SExpr::symbol("unquote"), sexpr])
    }

    pub fn lazy(x: SExpr) -> SExpr {
        SExpr::Lazy(Box::new(x))
    }

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

    pub fn is_lazy(&self) -> bool {
        match self {
            SExpr::Lazy(_) => true,
            _ => false
        }
    }

    pub fn is_unspecified(&self) -> bool {
        match self {
            SExpr::Unspecified => true,
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

    // Transform operations
    pub fn into_split(self) -> SResult<(SExpr, SExprs)> {
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
        exprs.push(parse_helper(&mut iter)?);
    }

    Ok(exprs)
}

fn parse_helper(iter: &mut Peekable<IntoIter<Token>>) -> SResult<SExpr> {
    match iter.peek() {
        Some(&Token::RParen) => bail!(UnexpectedToken => Token::RParen),
        Some(&Token::LParen) => {
            iter.next(); // Consume LParen

            // Check if empty list
            if iter.peek() == Some(&Token::RParen) {
                iter.next(); // Consume RParen
                return Ok(SExpr::List(vec![]));
            }

            let mut head: SExprs = vec![];
            while iter.peek() != Some(&Token::RParen) &&
                    iter.peek() != Some(&Token::Dot) {
                head.push(parse_helper(iter)?);
            }

            match iter.next() {
                Some(Token::Dot) => {
                    let tail = parse_helper(iter)?;
                    if iter.peek() != Some(&Token::RParen) {
                        let unexpected = iter.peek().unwrap().clone();
                        bail!(NotExpectedToken => unexpected, Token::RParen)
                    } else {
                        iter.next(); // Consume RParen
                        Ok(SExpr::DottedList(head, Box::new(tail)))
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
            Ok(SExpr::List(vec![SExpr::symbol("quote"), parse_helper(iter)?]))
        },
        Some(&Token::UnQuote) => {
            iter.next();
            Ok(SExpr::List(vec![SExpr::symbol("unquote"), parse_helper(iter)?]))
        },
        Some(&Token::QuasiQuote) => {
            iter.next();
            Ok(SExpr::List(vec![SExpr::symbol("quasiquote"), parse_helper(iter)?]))
        },
        Some(&Token::UnQuoteSplicing) => {
            iter.next();
            Ok(SExpr::List(vec![SExpr::symbol("unquote-splicing"), parse_helper(iter)?]))
        },
        Some(_) => {
            let y = iter.next().unwrap();
            Ok(SExpr::Atom(y))
        },
        None => serr!(FoundNothing)
    }
}
