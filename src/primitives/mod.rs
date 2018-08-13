pub mod lang;
pub mod lists;
pub mod numeric;
pub mod equivalence;
pub mod ordering;
pub mod conditionals;
pub mod io;

use env::EnvValues;
use procedure::ProcedureData;

pub fn env() -> EnvValues {
    environment! {
        "define"      => ProcedureData::new_primitive(lang::define),
        "set!"        => ProcedureData::new_primitive(lang::set),
        "lambda"      => ProcedureData::new_primitive(lang::lambda),
        "let"         => ProcedureData::new_primitive(lang::let_),
        "let*"        => ProcedureData::new_primitive(lang::let_star),
        "letrec"      => ProcedureData::new_primitive(lang::let_rec),
        "quote"       => ProcedureData::new_primitive(lang::quote),
        "quasiquote"  => ProcedureData::new_primitive(lang::quasiquote),
        "unquote"     => ProcedureData::new_primitive(lang::unquote),

        "eqv?"   => ProcedureData::new_primitive(equivalence::eqv),
        "eq?"    => ProcedureData::new_primitive(equivalence::eq),
        "equal?" => ProcedureData::new_primitive(equivalence::equal),

        "+"  => ProcedureData::new_primitive(|args| numeric::calc('+', args)),
        "-"  => ProcedureData::new_primitive(|args| numeric::calc('-', args)),
        "*"  => ProcedureData::new_primitive(|args| numeric::calc('*', args)),
        "/"  => ProcedureData::new_primitive(|args| numeric::calc('/', args)),
        "exact?"    => ProcedureData::new_primitive(numeric::exact),
        "inexact?"  => ProcedureData::new_primitive(numeric::inexact),

        "<"  => ProcedureData::new_primitive(ordering::lt),
        ">"  => ProcedureData::new_primitive(ordering::gt),
        "<=" => ProcedureData::new_primitive(ordering::lte),
        ">=" => ProcedureData::new_primitive(ordering::gte),
        "="  => ProcedureData::new_primitive(ordering::eq),

        "if"   => ProcedureData::new_primitive(conditionals::if_),
        "cond" => ProcedureData::new_primitive(conditionals::cond),
        "case" => ProcedureData::new_primitive(conditionals::case),
        "and"  => ProcedureData::new_primitive(conditionals::and),
        "or"   => ProcedureData::new_primitive(conditionals::or),

        "list" => ProcedureData::new_primitive(lists::list),
        "car"  => ProcedureData::new_primitive(lists::car),
        "cdr"  => ProcedureData::new_primitive(lists::cdr),

        "open-input-file" => ProcedureData::new_primitive(io::open_input_file),
        "read-line"       => ProcedureData::new_primitive(io::read_line)
    }
}
