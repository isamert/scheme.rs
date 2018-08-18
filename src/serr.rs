use std::fmt;
use std::error::Error;
use std::io;
use std::env;

use lexer::Token;
use parser::SExpr;

pub type SResult<T> = Result<T, SErr>;

#[derive(Debug)]
pub enum SErr {
    Generic(String),
    FoundNothing,
    EnvNotFound,
    UnexpectedForm(SExpr),
    UnexpectedToken(Token),
    NotExpectedToken(Token, Token),
    UnboundVar(String),
    NotAProcedure(SExpr),
    WrongArgCount(usize, usize),
    TypeMismatch(String, SExpr),
    WrongPort(/*proc: */String, /*port: */String),
    //TODO: what about Trace(String, Box<SErr>)

    // Converted errors
    IOErr(io::Error),
    VarErr(env::VarError)
}

impl fmt::Display for SErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            SErr::Generic(x) => x.to_string(),
            SErr::FoundNothing => "Expected some expression or token, found nothing.".to_string(),
            SErr::EnvNotFound => "Environment not found. (Probably an unbound variable)".to_string(),
            SErr::UnexpectedForm(x) => format!("Expression is in unexpected form: {}", x),
            SErr::UnexpectedToken(x) => format!("Not expected this token: {}", x),
            SErr::NotExpectedToken(x, y) => format!("Expected one of {}, found {}", x, y),
            SErr::UnboundVar(x) => format!("Unbound variable: {}", x),
            SErr::NotAProcedure(x) => format!("Wrong type to apply, not a procedure: {}", x),
            SErr::WrongArgCount(x, y) => format!("Wrong arg count; expected: {}, found: {}", x, y),
            SErr::TypeMismatch(x, y) => format!("Expected a {}, found this: {}", x, y),
            SErr::WrongPort(x, y) => format!("Can't apply function `{}` to a port type of {}", x, y),
            SErr::IOErr(x) => x.to_string(),
            SErr::VarErr(x) => x.to_string()
        };

        write!(f, "{}", &output)
    }
}

impl Error for SErr {
    fn description(&self) -> &str {
        match self {
            SErr::Generic(_) => "An error.",
            SErr::FoundNothing => "Expected some expression or token, found nothing.",
            SErr::EnvNotFound => "Environment not found. (Probably an unbound variable)",
            SErr::UnexpectedForm(_) => "Expression is in unexpected form.",
            SErr::UnexpectedToken(_) => "Unexpected token.",
            SErr::NotExpectedToken(_, _) => "Unexpected token.",
            SErr::UnboundVar(_) => "Unbound variable.",
            SErr::NotAProcedure(_) => "Not a procedure.",
            SErr::WrongArgCount(_, _) => "Wrong arg count.",
            SErr::TypeMismatch(_, _) => "Type mismatch.",
            SErr::WrongPort(_, _) => "Wrong type of port.",
            SErr::IOErr(x) => x.description(),
            SErr::VarErr(x) => x.description()
        }
    }
}

impl SErr {
    pub fn new_generic(s: &str) -> SErr {
        SErr::Generic(s.to_string())
    }

    pub fn new_unbound_var(s: &str) -> SErr {
        SErr::UnboundVar(s.to_string())
    }

    pub fn new_unexpected_form(x: &SExpr) -> SErr {
        SErr::UnexpectedForm(x.clone())
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

impl From<env::VarError> for SErr {
    fn from(error: env::VarError) -> Self {
        SErr::VarErr(error)
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
        return Err(SErr::Generic(($e).into()));
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(SErr::Generic(format!($fmt, $($arg)+)));
    };
    ($type:ident => $thing:expr) => {
        return Err(SErr::$type(($thing).into()));
    };
    ($type:ident => $($thing:expr),+) => {
        return Err(SErr::$type($(($thing).into()),+));
    };
}
