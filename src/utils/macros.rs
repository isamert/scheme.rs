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
            SExpr::Atom(Token::Symbol($e.into()))
        }
    }
);
