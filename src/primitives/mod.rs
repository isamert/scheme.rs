pub mod lang;
pub mod lists;
pub mod numeric;
pub mod equivalence;
pub mod ordering;
pub mod conditionals;
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

        "eqv?"   => equivalence::eqv,
        "eq?"    => equivalence::eq,
        "equal?" => equivalence::equal,

        "+"  => |args| numeric::calc('+', args),
        "-"  => |args| numeric::calc('-', args),
        "*"  => |args| numeric::calc('*', args),
        "/"  => |args| numeric::calc('/', args),
        "exact?"    => numeric::exact,
        "inexact?"  => numeric::inexact,

        "<"  => ordering::lt,
        ">"  => ordering::gt,
        "<=" => ordering::lte,
        ">=" => ordering::gte,
        "="  => ordering::eq,

        "if"   => conditionals::if_,
        "cond" => conditionals::cond,
        "case" => conditionals::case,
        "and"  => conditionals::and,
        "or"   => conditionals::or,

        "list" => lists::list,
        "cons" => lists::cons,
        "car"  => lists::car,
        "cdr"  => lists::cdr,

        "open-binary-input-file"  => io::open_binary_input_file,
        "open-binary-output-file" => io::open_binary_output_file,
        "open-input-file"  => io::open_input_file,
        "open-output-file" => io::open_output_file,
        "output-port?"     => io::input_port__,
        "input-port?"      => io::output_port__,
        "textual-port?"    => io::textual_port__,
        "binary-port?"     => io::binary_port__,
        "read-u8"          => io::read_u8,
        "read-line"        => io::read_line,
        "read-char"        => io::read_char,
        "read-all"         => io::read_all,
        "write"            => io::write,
        "write-string"     => io::write_string,
        "display"          => io::display,
        "newline"          => io::newline,
        "close-port"       => io::close_port,

        "load"         => system::load,
        "file-exists?" => system::file_exists,
        "delete-file"  => system::delete_file,
        "get-environment-variable"  => system::get_environment_variable,
        "get-environment-variables" => system::get_environment_variables
    }
}
