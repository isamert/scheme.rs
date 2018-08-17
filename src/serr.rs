use std::fmt;
use std::error::Error;
use std::io;

use lexer::Token;
use parser::SExpr;

pub type SResult<T> = Result<T, SErr>;

#[derive(Debug)]
pub enum SErr {
    Generic(String),
    FoundNothing,
    EnvNotFound,
    UnexpectedForm(String),
    UnexpectedToken(Token),
    NotExpectedToken(Token, Token),
    UnboundVar(String),
    NotAProcedure(SExpr),
    WrongArgCount(usize, usize),
    TypeMismatch(String, SExpr),
    IOErr(io::Error),
    //        proc,   port
    WrongPort(String, String),

    Trace(String, Box<SErr>)
}

impl fmt::Display for SErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR!")
    }
}

impl Error for SErr {
    fn description(&self) -> &str {
        ""
    }
}

impl SErr {
    pub fn new_generic(s: &str) -> SErr {
        SErr::Generic(s.to_string())
    }

    pub fn new_unbound_var(s: &str) -> SErr {
        SErr::UnboundVar(s.to_string())
    }

    pub fn new_unexpected_form(s: &str) -> SErr {
        SErr::UnexpectedForm(s.to_string())
    }

    pub fn new_id_not_found(s: &str) -> SErr {
        SErr::new_generic(&format!("Expected an identifer, found: {}", s))
    }

    pub fn new_expr_not_found(s: &str) -> SErr {
        SErr::new_generic(&format!("Expected an expression, found: {}", s))
    }
}

impl From<io::Error> for SErr {
    fn from(error: io::Error) -> Self {
        SErr::IOErr(error)
    }
}

#[macro_export]
macro_rules! serr {
    ($e:ident) => {
        return Err(SErr::$e);
    }
}

#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err(SErr::Generic($e.into()));
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(SErr::Generic(format!($fmt, $($arg)+)));
    };
    ($type:ident -> $thing:expr) => {
        return Err(SErr::$type($thing.to_string()));
    };
    ($type:ident => $thing:expr) => {
        return Err(SErr::$type($thing.into()));
    };
    ($type:ident => $($thing:expr),+) => {
        return Err(SErr::$type($($thing.into()),+));
    };
}
