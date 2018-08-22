pub mod lang;
pub mod equivalence;
pub mod boolean;
pub mod numeric;
pub mod ordering;
pub mod conditionals;
pub mod list;
pub mod io;
pub mod system;

use env::EnvValues;

pub fn env() -> EnvValues {
    environment! {
        "define"      => lang::define,
        "set!"        => lang::set,
        "lambda"      => lang::lambda,
        "let"         => lang::let_,
        "let*"        => lang::let_star,
        "letrec"      => lang::let_rec,
        "quote"       => lang::quote,
        "quasiquote"  => lang::quasiquote,
        "unquote"     => lang::unquote,

        "eqv?"   => equivalence::eqv_qm,
        "eq?"    => equivalence::eq_qm,
        "equal?" => equivalence::equal_qm,

        "not"       => boolean::not,
        "boolean?"  => boolean::boolean_qm,
        "boolean=?" => boolean::boolean_eq_qm,

        "+"  => |args| numeric::calc('+', args),
        "-"  => |args| numeric::calc('-', args),
        "*"  => |args| numeric::calc('*', args),
        "/"  => |args| numeric::calc('/', args),
        "exact?"    => numeric::exact_qm,
        "inexact?"  => numeric::inexact_qm,
        "remainder" => numeric::remainder,

        "<"  => ordering::lt,
        ">"  => ordering::gt,
        "<=" => ordering::lte,
        ">=" => ordering::gte,
        "="  => ordering::eq,

        "cond" => conditionals::cond,
        "case" => conditionals::case,
        "and"  => conditionals::and,
        "or"   => conditionals::or,

        "list"   => list::list,
        "cons"   => list::cons,
        "car"    => list::car,
        "cdr"    => list::cdr,
        "append" => list::append,
        "null?"  => list::null_qm,
        "pair?"  => list::pair_qm,
        "list?"  => list::list_qm,

        "load"         => system::load,
        "file-exists?" => system::file_exists_qm,
        "delete-file"  => system::delete_file,
        "system*"      => system::system_star,
        "get-environment-variable"  => system::get_environment_variable,
        "get-environment-variables" => system::get_environment_variables,

        "open-binary-input-file"  => io::open_binary_input_file,
        "open-binary-output-file" => io::open_binary_output_file,
        "open-input-file"  => io::open_input_file,
        "open-output-file" => io::open_output_file,
        "output-port?"     => io::input_port_qm,
        "input-port?"      => io::output_port_qm,
        "textual-port?"    => io::textual_port_qm,
        "binary-port?"     => io::binary_port_qm,
        "read"             => io::read,
        "read-u8"          => io::read_u8,
        "read-line"        => io::read_line,
        "read-char"        => io::read_char,
        "read-all"         => io::read_all,
        "write"            => io::write,
        "write-string"     => io::write_string,
        "display"          => io::display,
        "newline"          => io::newline,
        "close-port"       => io::close_port
    }
}
