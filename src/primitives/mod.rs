pub mod lang;
pub mod lists;
pub mod numeric;
pub mod ordering;
pub mod conditionals;

use env::EnvValues;
use procedure::ProcedureData;

pub fn env() -> EnvValues {
    environment! {
        "define"      => ProcedureData::new_primitive(lang::define),
        "lambda"      => ProcedureData::new_primitive(lang::lambda),
        "quote"       => ProcedureData::new_primitive(lang::quote),
        "quasiquote"  => ProcedureData::new_primitive(lang::quasiquote),
        "unquote"     => ProcedureData::new_primitive(lang::unquote),

        "+"  => ProcedureData::new_primitive(numeric::add),
        "-"  => ProcedureData::new_primitive(numeric::sub),
        "*"  => ProcedureData::new_primitive(numeric::mult),
        "/"  => ProcedureData::new_primitive(numeric::div),

        "<"  => ProcedureData::new_primitive(ordering::lt),
        ">"  => ProcedureData::new_primitive(ordering::gt),
        "<=" => ProcedureData::new_primitive(ordering::lte),
        ">=" => ProcedureData::new_primitive(ordering::gte),
        "="  => ProcedureData::new_primitive(ordering::eq),

        "if"   => ProcedureData::new_primitive(conditionals::if_),
        "cond" => ProcedureData::new_primitive(conditionals::cond),
        "case" => ProcedureData::new_primitive(conditionals::case),

        "list" => ProcedureData::new_primitive(lists::list),
        "car"  => ProcedureData::new_primitive(lists::car),
        "cdr"  => ProcedureData::new_primitive(lists::cdr)
    }
}
