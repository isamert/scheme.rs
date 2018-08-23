#[macro_export]
macro_rules! environment(
    { $($key:expr => $value:expr),* } => {
        {
            use env::EnvValues;
            use procedure::ProcedureData;
            let mut m = EnvValues::new();
            $(m.insert($key.to_string(), ProcedureData::new_primitive($value));)*
            m
        }
    };
);


#[macro_export]
macro_rules! slist(
    [ $($item:expr),* ] => {
        {
            use parser::SExpr;
            SExpr::List(vec![$($item),*])
        }
    };
);

#[macro_export]
macro_rules! sdottedlist(
    [ $($item:expr),* ; $last:expr ] => {
        {
            use parser::SExpr;
            SExpr::DottedList(vec![$($item),*], Box::new($last))
        }
    };
);

#[macro_export]
macro_rules! ssymbol(
    ($e: expr) => {
        {
            use parser::SExpr;
            use lexer::Token;
            SExpr::Atom(Token::Symbol($e.into()))
        }
    }
);


#[macro_export]
macro_rules! sbool(
    ($e: expr) => {
        {
            use parser::SExpr;
            use lexer::Token;
            SExpr::Atom(Token::Boolean($e.into()))
        }
    }
);


#[macro_export]
macro_rules! sint(
    ($e: expr) => {
        {
            use parser::SExpr;
            use lexer::Token;
            SExpr::Atom(Token::Integer($e.into()))
        }
    }
);


#[macro_export]
macro_rules! sstr(
    ($e: expr) => {
        {
            use parser::SExpr;
            use lexer::Token;
            SExpr::Atom(Token::Str($e.into()))
        }
    }
);

#[macro_export]
macro_rules! schr(
    ($e: expr) => {
        {
            use parser::SExpr;
            use lexer::Token;
            SExpr::Atom(Token::Chr($e.into()))
        }
    }
);

#[macro_export]
macro_rules! slazy(
    ($e: expr) => {
        {
            use parser::SExpr;
            SExpr::Lazy($e.into())
        }
    }
);

#[macro_export]
macro_rules! quasiquote(
    ($expr: expr) => {
        slist![ssymbol!("quasiquote"), $expr]
    }
);

#[macro_export]
macro_rules! quote(
    ($expr: expr) => {
        slist![ssymbol!("quote"), $expr]
    }
);

#[macro_export]
macro_rules! unquote(
    ($expr: expr) => {
        slist![ssymbol!("unquote"), $expr]
    }
);

#[macro_export]
macro_rules! unquote_splicing(
    ($expr: expr) => {
        slist![ssymbol!("unquote-splicing"), $expr]
    }
);
